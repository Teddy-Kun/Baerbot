use crate::{cli::SimpleResponse, BOT_NAME};
use eyre::Result;
use std::{process, str, sync::Arc};
use tracing::{debug, error, info, warn};
use twitch_irc::{
	login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport,
	TwitchIRCClient,
};
use twitch_oauth2::UserToken;

// Example from docs
pub async fn chat(token: &UserToken, responses: Arc<[SimpleResponse]>) -> Result<()> {
	let token = Arc::from(token.clone());
	let channel = token.login.as_str().to_string();
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
	let threaded_responses = responses.clone();
	let join_handle = tokio::spawn(async move {
		let channel: Arc<str> = Arc::from(token.login.as_str());
		while let Some(message) = incoming_messages.recv().await {
			handle_msg(
				thread_client.as_ref(),
				channel.to_string(),
				message,
				threaded_responses.clone().as_ref(),
			)
			.await
		}
	});

	client.join(login_name)?;
	if let Err(e) = client
		.say(channel, format!("{} initialized! 🧸", BOT_NAME).to_string())
		.await
	{
		error!("Error sending initial message: {:?}\nExiting", e);
		process::exit(1);
	};
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
	responses: &[SimpleResponse],
) {
	info!("Received message: {:?}", message.source());
	if message.source().params.len() == 2 {
		let (first, name) = message.source().params[0].split_at(1);
		if (first) != "#" {
			return;
		}

		let (first, command) = message.source().params[1].split_at(1);

		if first != "!" {
			return;
		}

		let command = command.trim();

		let mut response: Option<String> = None;

		for res in responses {
			if command == res.trigger.as_ref() {
				response = Some(res.response.to_string());
			} else {
				// workaround for twitch being big dumb dumb
				let cmd = command.as_bytes();
				let last_index = cmd.len();

				let (command, end) = cmd.split_at(last_index - 4);

				let command = unsafe { str::from_boxed_utf8_unchecked(command.into()) };
				let command = command.trim();

				if end == [0xf3, 0xa0, 0x80, 0x80] && command == res.trigger.as_ref() {
					// No I do not know why twitch does this sometimes
					response = Some(res.response.to_string());
				}
			}
		}

		debug!("{:?}", &response);

		info!("Received command from {}: {}", name, command);

		if response.is_none() {
			response =
				Some(format!("Received unknown command from {}: {}", name, command).to_string());
		}

		// TODO: handle result
		if let Err(e) = client.say(channel.clone(), response.unwrap()).await {
			warn!("Error sending message: {:?}", e);
		}
	}
}
