use std::fmt::Debug;

use obws::responses::general::Version;
use tokio::sync::RwLock;

use crate::{
	config::CONFIG,
	error::{Error, ErrorMsg, Result},
};
pub struct ObsData {
	websocket: obws::Client,
	version: Version,
}

impl Debug for ObsData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.version)
	}
}

static OBS_CLIENT: RwLock<Option<ObsData>> = RwLock::const_new(None);

pub async fn init_websocket() -> Result<()> {
	let cfg = CONFIG.read().obs.clone().unwrap_or_default();
	if !cfg.enable_ws {
		return Err(Error::from("OBS Websocket disabled").try_set_msg(ErrorMsg::ObsWS));
	}

	let client = obws::Client::connect(cfg.url, cfg.ws_port, cfg.password).await?;
	let version = client.general().version().await?;
	tracing::info!("Connected to OBS Version {}", version.obs_version);

	let mut cl = OBS_CLIENT.write().await;
	*cl = Some(ObsData {
		websocket: client,
		version,
	});

	Ok(())
}

pub async fn stop_ws() {
	_ = OBS_CLIENT.write().await.take();
}
