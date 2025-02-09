use std::sync::Arc;

use eyre::Result;
use tracing::{debug, info, warn};
use twitch_irc::{
	login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
	TwitchIRCClient,
};
use twitch_oauth2::UserToken;

// Example from docs
pub async fn chat(token: &UserToken) -> Result<()> {
	let token = Arc::from(token.clone());
	let login_name = token.login.as_str().to_owned();
	let oauth_token = token.access_token.as_str().to_owned();

	let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
		login_name.clone(),
		Some(oauth_token),
	));

	let (mut incoming_messages, client) =
		TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);
	let client = Arc::from(client);
	let thread_client = client.clone();
	let join_handle = tokio::spawn(async move {
		let channel: Arc<str> = Arc::from(token.login.as_str());
		while let Some(message) = incoming_messages.recv().await {
			handle_msg(thread_client.as_ref(), channel.to_string(), message).await
		}
	});

	client.join(login_name)?;
	debug!("after join");

	// keep the tokio executor alive.
	// If you return instead of waiting the background task will exit.
	join_handle.await?;

	Ok(())
}
async fn handle_msg(
	client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>,
	channel: String,
	message: ServerMessage,
) {
	info!("Received message: {:?}", message.source());
	if message.source().params.len() == 2 {
		let (first, name) = message.source().params[0].split_at(1);
		if (first) != "#" {
			return;
		}

		let msg = message.source().params[1].clone().into_boxed_str();
		if msg.get(0..1) != Some("!") {
			return;
		}
		info!("Received command from {}: {}", name, msg);
		// TODO: handle result
		if let Err(e) = client
			.say(
				channel.clone(),
				format!("Received command from {}: {}", name, msg),
			)
			.await
		{
			warn!("Error sending message: {:?}", e);
		}
	}
}
