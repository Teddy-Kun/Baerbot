use auth::load_token;
use color_eyre::eyre::Result;
use keyring::set_global_service_name;
use tts::setup_tts;
use twitch_api::HelixClient;
use twitch_oauth2::UserToken;
use std::sync::Arc;
use super::shared::cfg::get_merged_cfg;
use tracing::{debug, info, warn};

pub mod auth;
pub mod chat;
pub mod counter;
pub mod tts;

pub const BOT_NAME: &str = "Tedbot";

pub type TwitchClient = HelixClient<'static, reqwest::Client>;

pub fn new_client() -> Arc<TwitchClient> {
	Arc::new(HelixClient::default())
}

pub async fn print_channel_info(
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

pub fn install_tracing() {
	use tracing_error::ErrorLayer;
	use tracing_subscriber::prelude::*;
	use tracing_subscriber::{EnvFilter, fmt};

	let fmt_layer = fmt::layer().with_target(false);
	let filter_layer = EnvFilter::try_from_default_env()
		.or_else(|_| EnvFilter::try_new("info"))
		.unwrap();

	tracing_subscriber::registry()
		.with(filter_layer)
		.with(fmt_layer)
		.with(ErrorLayer::default())
		.init();
}


pub async fn start_service() -> Result<()> {
	install_tracing();
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
			if let Err(e) = setup_tts() {
				warn!("Failed to setup TTS: {:#?}", e);
			}
		}
	});

	chat::chat(&token, conf.clone()).await?;

	join_handle.await?;

	Ok(())
}
