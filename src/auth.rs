use std::time::Duration;

use eyre::Result;
use tokio::time::sleep;
use tracing::{debug, error, info};
use twitch_api::HelixClient;
use twitch_oauth2::{ClientId, DeviceUserTokenBuilder, Scope, UserToken};

async fn wait_for_code(dur: Duration, url: &str) {
	debug!("Got duration: {:#?}", dur);

	let handle = open::that_in_background(url).join();

	if let Err(e) = handle {
		error!("Failed to open browser: {:#?}", e);
	}

	// usually its 5 seconds, which I find too short
	sleep(dur).await;
}

pub async fn twitch_auth() -> Result<UserToken> {
	let client_id = ClientId::new("15xr4zw5ue7jxpbvt0jwwrwywqch9a".to_string());
	let scopes = vec![Scope::ChatEdit, Scope::ChatRead];

	let client: HelixClient<reqwest::Client> = HelixClient::default();

	let mut builder = DeviceUserTokenBuilder::new(client_id, scopes);
	let code = builder.start(&client).await?;

	debug!("code: {:#?}", code);

	let url = code.verification_uri.clone();

	let token = builder
		.wait_for_code(&client, |dur: Duration| wait_for_code(dur, &url))
		.await?;

	info!("Got token: {:#?}", token);

	Ok(token)
}
