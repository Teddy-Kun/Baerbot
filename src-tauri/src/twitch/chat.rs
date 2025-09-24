use tauri::async_runtime::{JoinHandle, spawn};
use twitch_irc::{
	ClientConfig, SecureTCPTransport, TwitchIRCClient, login::StaticLoginCredentials,
	message::ServerMessage,
};

use crate::{
	error::{Error, ErrorMsg},
	twitch::TwitchClient,
};

type IrcClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

pub async fn chat_listener(twitch_client: &mut TwitchClient) -> Result<JoinHandle<()>, Error> {
	tracing::debug!("Awaiting global twitch client read access");

	let user_tkn = twitch_client.get_token().ok_or(ErrorMsg::TokenGone)?;
	let username = user_tkn.login.to_string();

	let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
		username.clone(),
		Some(user_tkn.access_token.to_string()),
	));

	let (mut incoming_msg, client) = IrcClient::new(client_config);

	client.join(username.clone())?;

	tracing::debug!("Joined channel");

	let join_handle = spawn(async move {
		tracing::debug!("Started chat listener");
		loop {
			let msg = incoming_msg.recv().await;
			match msg {
				None => tracing::debug!("Received empty msg?"),
				Some(msg) => {
					if let Err(e) = handle_msg(msg, &client, username.clone()).await {
						tracing::error!("Error handling chat msg {e}");
					};
				}
			}
		}
	});

	Ok(join_handle)
}

async fn handle_msg(msg: ServerMessage, client: &IrcClient, username: String) -> Result<(), Error> {
	tracing::debug!("Message received: {:?}", msg.source());
	client.say(username, String::from("Test")).await?;

	Ok(())
}
