use eyre::Result;
use tracing::info;
use twitch_irc::{
	login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
};

// Example from docs
pub async fn chat() -> Result<()> {
	let config = ClientConfig::default();

	let (mut incoming_messages, client) =
		TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

	let join_handle = tokio::spawn(async move {
		while let Some(message) = incoming_messages.recv().await {
			info!("Received message: {:?}", message.source());
		}
	});

	// join a channel
	// This function only returns an error if the passed channel login name is malformed,
	// so in this simple case where the channel name is hardcoded we can ignore the potential
	// error with `unwrap`.
	client.join("teddy_kun".to_owned())?;

	// keep the tokio executor alive.
	// If you return instead of waiting the background task will exit.
	join_handle.await?;

	Ok(())
}
