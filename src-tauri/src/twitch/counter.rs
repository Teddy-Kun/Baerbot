use std::sync::Arc;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::async_runtime::RwLock;

#[derive(Debug, Deserialize, Serialize, Type)]
struct InnerCounter {
	counter: u32,
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

	pub async fn reset(&mut self) -> String {
		let mut inner = self.inner.write().await;
		inner.counter = 0;
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

impl Type for TwitchCounter {
	fn reference(
		type_map: &mut specta::TypeCollection,
		generics: &[specta::datatype::DataType],
	) -> specta::datatype::reference::Reference {
		<InnerCounter as Type>::reference(type_map, generics)
	}

	fn inline(
		type_map: &mut specta::TypeCollection,
		generics: specta::Generics,
	) -> specta::datatype::DataType {
		<InnerCounter as Type>::inline(type_map, generics)
	}
}
