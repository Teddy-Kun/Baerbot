use std::sync::{Arc, LazyLock};

use indexmap::IndexMap;
use rand::Rng;
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::Mutex;
use twitch_irc::{ClientConfig, login::StaticLoginCredentials, message::ServerMessage};

use crate::{
	error::{Error, ErrorMsg},
	twitch::{IrcClient, TWITCH_CLIENT, TwitchClient, actions::get_action},
	utils::{NAME_CAPITALIZED, get_unix},
};

static ACTIVE_CHATTERS: LazyLock<Mutex<IndexMap<Box<str>, u64>>> =
	LazyLock::new(|| Mutex::new(IndexMap::new()));

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

	twitch_client.chat_client = Some(client.clone());

	let join_handle = spawn(async move {
		tracing::debug!("Started chat listener");
		loop {
			let msg = incoming_msg.recv().await;
			match msg {
				None => tracing::debug!("Received empty msg?"),
				Some(msg) => {
					spawn(async move {
						if let Err(e) = handle_msg(msg).await {
							tracing::error!("Error handling chat msg {e}");
						};
					});
				}
			}
		}
	});

	client.join(username.clone())?;
	client
		.say(username, format!("{} initialized! 🧸", NAME_CAPITALIZED))
		.await?;

	Ok(join_handle)
}

fn register_active_chatter(name: Box<str>) {
	spawn(async move {
		let unix = get_unix();
		let mut active_chatters = ACTIVE_CHATTERS.lock().await;
		active_chatters.insert(name, unix);
	});
}

pub async fn is_chatter_active(name: &str) -> bool {
	let unix = get_unix();
	let mut active_chatters = ACTIVE_CHATTERS.lock().await;
	// remove all chatters that are no longer active
	// they count as active if they have chatted in the last 5m
	active_chatters.retain(|_, time| (*time - unix) > 5 * 60);
	active_chatters.contains_key(name)
}

pub async fn get_random_chatter() -> Option<Box<str>> {
	let unix = get_unix();
	let mut active_chatters = ACTIVE_CHATTERS.lock().await;
	active_chatters.retain(|_, time| (*time - unix) > 5 * 60);

	if active_chatters.is_empty() {
		return None;
	}

	let mut rng = rand::rng();
	let i = rng.random_range(0..active_chatters.len());
	active_chatters.get_index(i).map(|(s, _)| s).cloned()
}

async fn handle_msg(server_msg: ServerMessage) -> Result<(), Error> {
	tracing::debug!("Message received: {:?}", server_msg.source().params);

	let params = &server_msg.source().params;

	let chatter_name = match params.first() {
		None => return Ok(()),
		Some(p) => p.as_str(),
	};
	let (prefix, msg) = match params.get(1) {
		None => return Ok(()),
		Some(p) => match p.split_at_checked(1) {
			None => return Ok(()),
			Some(s) => s,
		},
	};

	register_active_chatter(Box::from(chatter_name));

	if (msg.contains('.') || msg.contains("dot")) && msg.contains("cheap viewers") {
		_ = TWITCH_CLIENT
			.read()
			.await
			.ban_user(chatter_name, "bot detected", None)
			.await;
		return Ok(());
	}

	if prefix != "!" {
		return Ok(());
	}

	tracing::debug!("cmd attempt");

	let mut split = msg.split(' ');

	let cmd = match split.next() {
		None => return Ok(()),
		Some(m) => m,
	};

	let mut action = match get_action(cmd.to_lowercase().as_str()).await {
		Some(a) => a,
		None => return Ok(()),
	};

	if !action.allow_use() {
		return Ok(()); // action is still on cooldown
	}

	let msg = split.next();
	tracing::debug!("action: {action:?}; msg: {msg:?}");

	action.exec.exec(chatter_name, msg).await;

	Ok(())
}
