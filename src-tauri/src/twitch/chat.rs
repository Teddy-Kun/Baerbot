use std::sync::Arc;

use tauri::async_runtime::{JoinHandle, spawn};
use twitch_irc::{
	ClientConfig, SecureTCPTransport, TwitchIRCClient, login::StaticLoginCredentials,
	message::ServerMessage,
};

use crate::{
	error::{Error, ErrorMsg},
	statics::NAME_CAPITALIZED,
	twitch::{
		TwitchClient,
		actions::{self, Exec},
	},
};

type IrcClient = TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>;

pub async fn chat_listener(twitch_client: &mut TwitchClient) -> Result<JoinHandle<()>, Error> {
	tracing::debug!("Awaiting global twitch client read access");

	let user_tkn = twitch_client
		.get_token()
		.ok_or(ErrorMsg::TokenGone)?
		.clone();
	let username = user_tkn.login.to_string();

	let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
		username.clone(),
		Some(user_tkn.access_token.clone().take()), // to string doesn't work because it redacts the token so that you don't print it on accident
	));

	let (mut incoming_msg, client) = IrcClient::new(client_config);

	let client = Arc::new(client);
	client.join(username.clone())?;
	client
		.say(
			username.clone(),
			format!("{} initialized! 🧸", NAME_CAPITALIZED.as_str()),
		)
		.await?;

	let join_handle = spawn(async move {
		tracing::debug!("Started chat listener");
		loop {
			let msg = incoming_msg.recv().await;
			match msg {
				None => tracing::debug!("Received empty msg?"),
				Some(msg) => {
					let clone = client.clone();
					let username_clone = username.clone();
					spawn(async move {
						if let Err(e) = handle_msg(msg, clone.as_ref(), username_clone).await {
							tracing::error!("Error handling chat msg {e}");
						};
					});
				}
			}
		}
	});

	Ok(join_handle)
}

async fn handle_msg(
	server_msg: ServerMessage,
	client: &IrcClient,
	username: String,
) -> Result<(), Error> {
	tracing::debug!("Message received: {:?}", server_msg.source().params);

	let params = &server_msg.source().params;

	if params.len() != 2 {
		return Ok(());
	}

	let (prefix, msg) = params[1].split_at(1);

	if prefix != "!" {
		return Ok(());
	}

	tracing::debug!("cmd attempt");

	let msg: Vec<&str> = msg.split(' ').collect();

	let action = match actions::get_action(msg[0]).await {
		Some(a) => a,
		None => return Ok(()),
	};

	tracing::debug!("action: {action:?}");

	match action.exec {
		Exec::ChatMsg(msg) => _ = client.say(username, msg.to_string()).await,
		_ => {
			// TODO
		}
	};

	Ok(())
}
