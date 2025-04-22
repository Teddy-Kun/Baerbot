use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub struct SimpleResponse {
	pub trigger: String,
	pub response: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type)]
pub struct TimedShoutout {
	pub seconds: u16,
	pub message: String,
}
