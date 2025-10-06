use twitch_api::{
	eventsub::{Transport, channel::ChannelPointsCustomRewardRedemptionAddV1},
	helix::points::CustomReward,
};

use crate::{
	error::{Error, ErrorMsg},
	twitch::{
		TwitchClient,
		actions::{Trigger, get_action},
	},
};

impl TwitchClient {
	pub async fn update_redeems(&self) -> Result<Vec<CustomReward>, Error> {
		let user_token = match &self.token {
			None => return Err(Error::new(ErrorMsg::TokenGone)),
			Some(t) => t,
		};

		let info = match &self.user_info {
			None => return Err(Error::new(ErrorMsg::UsernameGone)),
			Some(i) => i,
		};

		tracing::debug!("user info? {:?}", info);

		let redeems = self
			.client
			.get_all_custom_rewards(&info.id, false, user_token.as_ref())
			.await?;

		redeems
			.iter()
			.for_each(|redeem| tracing::debug!("redeem '{}: {}'", redeem.title, redeem.cost));
		Ok(redeems)
	}

	pub async fn sub_new_redeems(&self) -> Result<(), Error> {
		let user_token = match &self.token {
			None => return Err(Error::new(ErrorMsg::TokenGone)),
			Some(t) => t,
		};

		let info = match &self.user_info {
			None => return Err(Error::new(ErrorMsg::UsernameGone)),
			Some(i) => i,
		};

		let ws_id = match &self.websocket_id {
			None => return Err(Error::new(ErrorMsg::WebSocketSetup)),
			Some(w) => w,
		};

		let event = ChannelPointsCustomRewardRedemptionAddV1::broadcaster_user_id(info.id.clone());

		let trans = Transport::websocket(ws_id);

		let sub = self
			.client
			.create_eventsub_subscription(event, trans, user_token.as_ref())
			.await?;

		// do we need this response?
		tracing::debug!("Setup Redeem Subscription: {sub:#?}");

		Ok(())
	}
}

pub async fn exec_redeem(chatter_name: &str, id: &str, prompt: &str) {
	if let Some(action) = get_action(id).await
		&& let Trigger::Redeem(_) = action.trigger
	{
		let prompt = match prompt.len() {
			0 => None,
			_ => Some(prompt),
		};
		action.exec.exec(chatter_name, prompt).await;
	}
}
