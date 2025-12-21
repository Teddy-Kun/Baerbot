use std::{fmt::Debug, sync::Arc};

use obws::responses::general::Version;
use tokio::sync::RwLock;

use crate::{
	config::{CONFIG, ObsConfig},
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

static OBS_CLIENT: RwLock<Option<Arc<ObsData>>> = RwLock::const_new(None);

pub async fn init_websocket() -> Result<()> {
	let cfg = CONFIG.read().obs.clone().unwrap_or_default();
	if !cfg.enable_ws.unwrap_or(false) {
		return Err(Error::from("OBS Websocket disabled").try_set_msg(ErrorMsg::ObsWS));
	}

	let url = cfg.url.unwrap_or(ObsConfig::default().url.unwrap());
	let ws_port = cfg.ws_port.unwrap_or(ObsConfig::default().ws_port.unwrap());
	let password = cfg.password;
	let client = obws::Client::connect(url, ws_port, password).await?;

	let version = client.general().version().await?;
	tracing::info!("Connected to OBS Version {}", version.obs_version);

	let mut cl = OBS_CLIENT.write().await;
	*cl = Some(Arc::new(ObsData {
		websocket: client,
		version,
	}));

	Ok(())
}

pub async fn mute_input() -> Result<()> {
	let reader = match OBS_CLIENT.read().await.clone() {
		Some(r) => r,
		None => return Ok(()),
	};
	let inputs = reader.websocket.inputs().list(None).await?;
	let futures = inputs.iter().map(async |i| {
		return (
			reader
				.websocket
				.inputs()
				.set_muted(i.id.clone().into(), true)
				.await,
			&i.id,
		);
	});

	for req in futures {
		if let (Err(e), input_id) = req.await {
			tracing::warn!("Couldn't mute input '{}': {}", input_id.name, e)
		}
	}

	Ok(())
}

pub async fn stop_ws() {
	_ = OBS_CLIENT.write().await.take();
}
