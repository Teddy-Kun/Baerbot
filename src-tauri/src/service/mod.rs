use super::shared::{cfg::get_merged_cfg, data::BOT_NAME};
use auth::load_token;
use color_eyre::eyre::Result;
use keyring::set_global_service_name;
use log::{debug, info, warn};
use std::sync::Arc;
use tts::setup_tts;
use twitch_api::HelixClient;
use twitch_oauth2::UserToken;

mod auth;
mod chat;
mod counter;
mod tts;

type TwitchClient = HelixClient<'static, reqwest::Client>;

fn new_client() -> Arc<TwitchClient> {
	Arc::new(HelixClient::default())
}

async fn print_channel_info(
	client: &TwitchClient,
	username: &str,
	token: &UserToken,
) -> Result<()> {
	debug!("token user: {:#?}", token.user_id);

	info!(
		"Channel: {:?}",
		client.get_channel_from_login(username, token).await?
	);

	Ok(())
}

pub async fn start_service() -> Result<()> {
	color_eyre::install()?;

	set_global_service_name(BOT_NAME);

	let conf = Arc::new(get_merged_cfg()?);

	let client = new_client();

	let token = match load_token(client.as_ref(), &conf).await {
		Ok(token) => token,
		Err(_) => {
			debug!("Failed to load token, authenticating");
			auth::twitch_auth(&conf).await?
		}
	};

	print_channel_info(client.as_ref(), conf.get_username().as_ref(), &token).await?;

	let conf_clone = conf.clone();
	let join_handle = tokio::task::spawn_blocking(move || {
		if conf_clone.tts_chance > 0.0 {
			if let Err(e) = setup_tts(conf_clone.as_ref()) {
				warn!("Failed to setup TTS: {:#?}", e);
			}
		}
	});

	chat::chat(&token, conf.clone()).await?;

	join_handle.await?;

	Ok(())
}
