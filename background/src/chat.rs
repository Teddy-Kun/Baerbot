use crate::{BOT_NAME, counter};
use eyre::Result;
use shared::cfg::Config;
use std::{process, str, sync::Arc};
use tracing::{debug, error, info, warn};
use twitch_irc::{
	ClientConfig, SecureTCPTransport, TwitchIRCClient, login::StaticLoginCredentials,
	message::ServerMessage,
};
use twitch_oauth2::UserToken;

// Example from docs
pub async fn chat(token: &UserToken, config: Arc<Config>) -> Result<()> {
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
	let thread_config = config.clone();
	let join_handle = tokio::spawn(async move {
		let channel: Arc<str> = Arc::from(token.login.as_str());
		while let Some(message) = incoming_messages.recv().await {
			handle_msg(
				thread_client.as_ref(),
				channel.to_string(),
				message,
				thread_config.as_ref(),
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
	config: &Config,
) {
	info!("Received message: {:?}", message.source());

	let mut msg_len = message.source().params.len();

	if msg_len < 2 {
		return;
	}

	let (first, name) = message.source().params[0].split_at(1);
	if (first) != "#" {
		return;
	}

	let (first, command) = message.source().params[1].split_at(1);

	if first != "!" {
		return;
	}

	let command = command.trim();

	info!("Received command from {}: {}", name, command);

	let splitted: Box<[&str]> = command.split_ascii_whitespace().collect();
	msg_len = splitted.len();

	let mut response: Option<String> = None;

	match msg_len {
		1 => response = get_response(command, config),
		2 => {
			let command = splitted[0];
			match handle_counters(command.into(), splitted[1].into(), None, config).await {
				Ok(res) => response = Some(res),
				Err(e) => {
					if let Some(e) = e {
						response = Some(e);
					}
				}
			};
		}
		3 => {
			let command = splitted[0];
			match handle_counters(
				command.into(),
				splitted[1].into(),
				Some(splitted[2].into()),
				config,
			)
			.await
			{
				Ok(res) => response = Some(res),
				Err(e) => {
					if let Some(e) = e {
						response = Some(e);
					}
				}
			};
		}
		_ => {
			// TODO: error msg
			return;
		}
	}

	if let Some(response) = response {
		if let Err(e) = client.say(channel.clone(), response).await {
			warn!("Error sending message: {:?}", e);
		}
	}
}

fn get_response(command: &str, config: &Config) -> Option<String> {
	let mut command: Box<str> = Box::from(command);
	// workaround for twitch being big dumb dumb
	let cmd = command.as_bytes();
	let last_index = cmd.len();

	let (cmd, end) = cmd.split_at(last_index - 4);

	if end == [0xf3, 0xa0, 0x80, 0x80] {
		command = unsafe { str::from_boxed_utf8_unchecked(cmd.into()) };
	}

	if command.as_ref() == "tts" {
		// TODO: global tts queue
		return None;
	}

	for res in config.simple_responses.iter() {
		if command.as_ref() == res.trigger.as_ref() {
			return Some(res.response.to_string());
		}
	}

	None
}

// if this return `Ok` it means the counter updated
// if it returns an empty Err it means the counter does not exist
// if it returns a filled Err it means the counter does exist but something went wrong
async fn handle_counters(
	counter: Arc<str>,
	command: Arc<str>,
	value: Option<Arc<str>>,
	config: &Config,
) -> Result<String, Option<String>> {
	if !config.counters.contains(&counter) {
		return Err(None);
	}

	let res: isize = match command.to_lowercase().as_str() {
		"set" => {
			let value = match value {
				None => return Err(Some(String::from("No value provided"))),
				Some(value) => match value.parse::<isize>() {
					Err(_) => return Err(Some(String::from("Value is not a number"))),
					Ok(v) => v,
				},
			};
			counter::set(counter.clone(), value).await;

			value
		}
		"get" => counter::get(counter.clone()).await,
		"inc" => counter::increase(counter.clone()).await,
		"dec" => counter::decrease(counter.clone()).await,
		_ => {
			return Err(Some(String::from("Invalid counter action")));
		}
	};

	return Ok(format!("{counter} is now {res}"));
}
