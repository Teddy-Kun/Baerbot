use color_eyre::eyre::Result;
use tracing::{debug, info};
use twitch_api::HelixClient;
use twitch_oauth2::UserToken;

pub mod auth;
pub mod chat;
pub mod cli;

pub async fn print_channel_info(username: &str, token: &UserToken) -> Result<()> {
	let client: HelixClient<reqwest::Client> = HelixClient::default();

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
	use tracing_subscriber::{fmt, EnvFilter};

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
