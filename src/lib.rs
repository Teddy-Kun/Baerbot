use cli::Config;
use color_eyre::eyre::Result;
use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, UserToken};

pub mod cli;

pub async fn init() -> Result<()> {
	let conf = Config::get()?;

	dbg!(&conf);

	// Create the HelixClient, which is used to make requests to the Twitch API
	let client: HelixClient<reqwest::Client> = HelixClient::default();
	// Create a UserToken, which is used to authenticate requests.
	let token = UserToken::from_token(&client, AccessToken::from(conf.token.as_ref())).await?;

	println!(
		"Channel: {:?}",
		client
			.get_channel_from_login(conf.username.as_ref(), &token)
			.await?
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
