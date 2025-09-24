use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Serialize)]
struct InnerCounter {
	counter: i64,
	template: String,
}

#[derive(Debug, Clone)]
pub struct TwitchCounter {
	inner: Arc<RwLock<InnerCounter>>,
}

impl TwitchCounter {
	pub async fn add(&mut self) -> String {
		let mut inner = self.inner.write().await;
		inner.counter += 1;
		format_counter(&inner)
	}

	pub async fn sub(&mut self) -> String {
		let mut inner = self.inner.write().await;
		inner.counter -= 1;
		format_counter(&inner)
	}
}

fn format_counter(counter: &InnerCounter) -> String {
	counter
		.template
		.clone()
		.replace("{counter}", counter.counter.to_string().as_str())
}

impl Serialize for TwitchCounter {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let inner = self.inner.blocking_read();
		inner.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for TwitchCounter {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let inner = InnerCounter::deserialize(deserializer)?;

		Ok(TwitchCounter {
			inner: Arc::new(RwLock::new(inner)),
		})
	}
}
