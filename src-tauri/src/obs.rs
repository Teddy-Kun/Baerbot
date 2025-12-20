use std::fmt::Debug;

use obws::responses::general::Version;
use tokio::sync::{OnceCell, RwLock};

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

pub static OBS_CLIENT: OnceCell<RwLock<ObsData>> = OnceCell::const_new();

pub async fn init_websocket() -> Result<()> {
	let cfg = CONFIG.read().obs.clone();
	if !cfg.enable {
		return Err(Error::from("OBS Websocket disabled").try_set_msg(ErrorMsg::Obs));
	}

	let client = obws::Client::connect(cfg.url, cfg.port, cfg.password).await?;
	let version = client.general().version().await?;
	tracing::info!("Connected to OBS Version {}", version.obs_version);

	OBS_CLIENT.set(RwLock::new(ObsData {
		websocket: client,
		version,
	}))?;

	Ok(())
}
