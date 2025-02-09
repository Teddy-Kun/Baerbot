use eyre::Result;
use tracing::info;
use twitch_irc::{
	login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};
use twitch_oauth2::UserToken;

// Example from docs
pub async fn chat(token: UserToken) -> Result<()> {
	let login_name = token.login.as_str().to_owned();
	let oauth_token = token.access_token.as_str().to_owned();

	let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
		login_name.clone(),
		Some(oauth_token),
	));

	let (mut incoming_messages, client) =
		TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

	let join_handle = tokio::spawn(async move {
		while let Some(message) = incoming_messages.recv().await {
			info!("Received message: {:?}", message.source());
		}
	});

	client.join(login_name)?;

	// keep the tokio executor alive.
	// If you return instead of waiting the background task will exit.
	join_handle.await?;

	Ok(())
}
