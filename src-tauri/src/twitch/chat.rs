use std::sync::{Arc, LazyLock};

use indexmap::IndexMap;
use rand::Rng;
use tauri::async_runtime::{JoinHandle, spawn};
use tokio::sync::Mutex;
use twitch_irc::{ClientConfig, login::StaticLoginCredentials, message::ServerMessage};

use crate::{
	error::{Error, ErrorMsg},
	twitch::{IrcClient, TwitchClient, actions::get_action},
	utils::{NAME_CAPITALIZED, get_unix},
};

static ACTIVE_CHATTERS: LazyLock<Mutex<IndexMap<String, u64>>> =
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
		.say(
			username,
			format!("{} initialized! 🧸", NAME_CAPITALIZED.as_str()),
		)
		.await?;

	Ok(join_handle)
}

fn register_active_chatter(name: String) {
	let unix = get_unix();
	spawn(async move {
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

pub async fn get_random_chatter() -> Option<String> {
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

	if params.is_empty() {
		return Ok(());
	}

	register_active_chatter(params[0].clone());

	if params.len() != 2 {
		return Ok(());
	}

	let (prefix, msg) = params[1].split_at(1);

	if prefix != "!" {
		return Ok(());
	}

	tracing::debug!("cmd attempt");

	let mut split = msg.split(' ');

	let msg = match split.next() {
		None => return Ok(()),
		Some(m) => m,
	};

	let action = match get_action(msg.to_lowercase().as_str()).await {
		Some(a) => a,
		None => return Ok(()),
	};

	if !action.allow_use() {
		return Ok(()); // action is still on cooldown
	}

	tracing::debug!("action: {action:?}");

	action.exec.exec().await;

	Ok(())
}
