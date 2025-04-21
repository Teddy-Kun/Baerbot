use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleResponse {
	pub trigger: Arc<str>,
	pub response: Arc<str>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimedShoutout {
	pub seconds: u16,
	pub message: Arc<str>,
}
