use futures::StreamExt;
use tokio::spawn;
use tokio_tungstenite::connect_async;
use twitch_api::eventsub::{Event, EventsubWebsocketData, Message};

use crate::{
	error::Error,
	twitch::{TWITCH_CLIENT, TwitchClient, redeems::exec_redeem},
};

impl TwitchClient {
	pub async fn setup_websocket() -> Result<(), Error> {
		let (socket, resp) = connect_async("wss://eventsub.wss.twitch.tv/ws").await?;

		if let Some(b) = resp.body().as_deref() {
			let s = str::from_utf8(b);
			tracing::info!("WebSocket response:\n{s:?}");
		}

		let (_, mut read) = socket.split();

		spawn(async move {
			while let Some(Ok(msg)) = read.next().await {
				let msg = match msg.into_text() {
					Err(_) => continue,
					Ok(m) => m,
				};

				let event = match Event::parse_websocket(msg.as_str()) {
					Err(_) => continue,
					Ok(e) => e,
				};

				{
					let mut tw_client = TWITCH_CLIENT.write().await;
					tw_client.websocket_last_event = Some(std::time::SystemTime::now());
					// auto-drop TWITCH_CLIENT lock
				}

				match event {
					EventsubWebsocketData::Welcome {
						metadata: _,
						payload,
					} => {
						let mut twitch_client = TWITCH_CLIENT.write().await;
						twitch_client.websocket_id = Some(payload.session.id.into());

						if let Err(e) = twitch_client.sub_new_redeems().await {
							tracing::error!("Error setting up redeem subscription: {e}");
						};
					}
					EventsubWebsocketData::Notification {
						metadata: _,
						payload,
					} => match payload {
						Event::ChannelPointsCustomRewardRedemptionAddV1(payload) => {
							match payload.message {
								Message::Notification(event) => {
									tracing::info!(
										"{} redeemed: {} ({}); {}",
										event.user_name,
										event.reward.title,
										event.reward.id,
										event.reward.prompt
									);

									// TODO: auto-remove event from rewards queue as successfull or reject them
									_ = exec_redeem(
										event.reward.id.as_str(),
										event.reward.prompt.as_str(),
									);
								}
								p => tracing::warn!("Unknown event msg:\n{p:#?}"),
							}
						}
						_ => tracing::warn!("Unknown notification:\n{payload:#?}"),
					},
					EventsubWebsocketData::Keepalive {
						metadata: _,
						payload: _,
					} => {
						// do nothing
						// we track the heartbeat timer further up
					}
					_ => tracing::warn!("Unhandled websocket event:\n{:#?}", event),
				}
			}
		});

		Ok(())
	}
}
