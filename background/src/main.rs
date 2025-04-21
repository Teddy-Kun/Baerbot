use color_eyre::eyre::Result;
use keyring::set_global_service_name;
use shared::cfg::get_merged_cfg;
use std::sync::Arc;
use tedbot::{
	BOT_NAME,
	auth::{self, load_token},
	chat, install_tracing, new_client, print_channel_info,
	tts::setup_tts,
};
use tracing::{debug, warn};

#[tokio::main]
async fn main() -> Result<()> {
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
