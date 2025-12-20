use std::{fmt::Debug, sync::Arc};

use tokio::{fs, io::AsyncWriteExt, net::TcpListener, spawn, sync::RwLock, task::JoinHandle};

use crate::{
	config::CONFIG,
	error::{Error, Result},
};

const DEFAULT_HTMLL: &str = include_str!("../../assets/index.html");

pub struct ObsOverlay {
	server: JoinHandle<!>,
	html: Arc<str>,
}

impl Drop for ObsOverlay {
	fn drop(&mut self) {
		self.server.abort(); // abort the infinite loop before exiting
	}
}

impl Debug for ObsOverlay {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.html)
	}
}

static OBS_OVERLAY: RwLock<Option<ObsOverlay>> = RwLock::const_new(None);

/// Hosts the probably simplest HTTP server possible in Rust
/// Returns a single HTML document read from disk upon initialization
pub async fn init_overlay() -> Result<()> {
	let cfg = CONFIG.read().obs.clone().unwrap_or_default();
	if !cfg.enable_host {
		return Err(Error::from("OBS Host disabled"));
	}

	let html: Arc<str> = match fs::read_to_string("index.html").await {
		Ok(s) => s.into(),
		Err(_) => DEFAULT_HTMLL.into(),
	};
	let loop_clone = html.clone();

	let listener = TcpListener::bind(format!("127.0.0.1:{}", cfg.host_port)).await?;

	let join_handle = spawn(async move {
		loop {
			match listener.accept().await {
				Ok((mut socket, _addr)) => {
					let response = format!(
						"HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
						loop_clone.len(),
						loop_clone
					);
					if let Err(e) = socket.write_all(response.as_bytes()).await {
						tracing::error!("Error while responding to OBS overlay connection: {e}")
					}
				}
				Err(e) => {
					tracing::error!("Error while listening for OBS overlay connection: {e}")
				}
			}
		}
	});

	let mut overlay = OBS_OVERLAY.write().await;
	*overlay = Some(ObsOverlay {
		server: join_handle,
		html,
	});

	Ok(())
}

pub async fn stop_overlay() {
	_ = OBS_OVERLAY.write().await.take();
}
