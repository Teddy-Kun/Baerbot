use std::{
	borrow::Borrow,
	collections::HashMap,
	fs::{self, OpenOptions, create_dir_all, read_dir, remove_file},
	io::Write,
	ops::Deref,
	sync::{Arc, LazyLock},
};

use futures::{StreamExt, stream::FuturesUnordered};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::async_runtime::RwLock;

use crate::{error::Error, statics::CFG_DIR_PATH, twitch::counter::TwitchCounter};

static ACTION_TABLE: LazyLock<RwLock<HashMap<ArcStr, Action>>> = LazyLock::new(|| {
	let m = init_map().unwrap_or_default();
	RwLock::new(m)
});

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
// wrapper around Arc<str> so that we can implement Type by hand, until builtin support is in specta
pub struct ArcStr(Arc<str>);

impl Borrow<str> for ArcStr {
	fn borrow(&self) -> &str {
		self.0.borrow()
	}
}

impl Deref for ArcStr {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl From<&str> for ArcStr {
	fn from(value: &str) -> Self {
		Self(Arc::from(value))
	}
}

impl From<Arc<str>> for ArcStr {
	fn from(value: Arc<str>) -> Self {
		Self(value)
	}
}

impl Type for ArcStr {
	fn inline(
		type_map: &mut specta::TypeCollection,
		generics: specta::Generics,
	) -> specta::datatype::DataType {
		<String as Type>::inline(type_map, generics)
	}

	fn reference(
		type_map: &mut specta::TypeCollection,
		generics: &[specta::datatype::DataType],
	) -> specta::datatype::reference::Reference {
		<String as Type>::reference(type_map, generics)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Trigger {
	Command(ArcStr),
	Redeem(ArcStr),
}

impl Trigger {
	fn get_inner(&self) -> &ArcStr {
		match self {
			Trigger::Command(s) | Trigger::Redeem(s) => s,
		}
	}
}

impl Deref for Trigger {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		self.get_inner()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Exec {
	ChatMsg(ArcStr),
	Reply(ArcStr),
	Counter(TwitchCounter),
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Action {
	pub trigger: Trigger,
	pub exec: Exec,
}

pub async fn get_action(key: &str) -> Option<Action> {
	let table = ACTION_TABLE.read().await;
	table.get(key).cloned()
}

pub async fn add_action(action: Action) {
	let mut table = ACTION_TABLE.write().await;
	table.insert(action.trigger.get_inner().clone(), action);
	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = save_actions_inner(&table).await {
		tracing::error!("Error saving actions: {e}")
	};
}

pub async fn drop_action(key: &str) {
	let mut table = ACTION_TABLE.write().await;
	table.remove(key);
	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = delete_action_from_fs(key) {
		tracing::error!("Error deleting action from fs: {e}")
	};
}

pub async fn save_actions() -> Result<(), Error> {
	let table = ACTION_TABLE.read().await;
	save_actions_inner(&table).await
}

pub async fn get_all_actions() -> Vec<Action> {
	let table = ACTION_TABLE.read().await;
	table.values().map(|a| a.clone()).collect()
}

async fn save_actions_inner(table: &HashMap<ArcStr, Action>) -> Result<(), Error> {
	let mut p = CFG_DIR_PATH.clone();
	p.push("actions");

	let v: Vec<Action> = table.values().map(|a| a.clone()).collect();

	tracing::debug!("actions dir {p:?}");

	create_dir_all(&p)?;

	let mut futures = FuturesUnordered::new();

	for action in v {
		let mut p = p.clone();

		let handle = tokio::spawn(async move {
			p.push(format!("{}.toml", action.trigger.deref()));

			tracing::info!("action path {p:?}");

			let s = toml::to_string_pretty(&action)?;

			let mut f = OpenOptions::new()
				.create(true)
				.write(true)
				.truncate(true)
				.open(p)?;
			f.write_all(s.as_bytes())?;

			Ok::<(), Error>(())
		});

		futures.push(handle);
	}

	while let Some(res) = futures.next().await {
		if let Err(e) = res {
			tracing::warn!("Couldn't save action file {e}");
		}
	}

	Ok(())
}

fn delete_action_from_fs(key: &str) -> Result<(), Error> {
	let mut p = CFG_DIR_PATH.clone();
	p.push("actions");

	if p.is_dir() {
		for entry in read_dir(&p)? {
			let entry = entry?;
			let p = entry.path();
			let target_name = format!("{key}.toml");
			if p.is_file()
				&& let Some(filename) = p.file_name()
				&& filename == target_name.as_str()
			{
				remove_file(&p)?;
				break;
			}
		}
	}

	Ok(())
}

fn init_map() -> Result<HashMap<ArcStr, Action>, Error> {
	let mut m = HashMap::new();

	let mut p = CFG_DIR_PATH.clone();
	p.push("actions");

	if p.is_dir() {
		for entry in read_dir(&p)? {
			let entry = entry?;
			let p = entry.path();
			if p.is_file()
				&& let Some(extension) = p.extension()
				&& extension == "toml"
			{
				let content = fs::read_to_string(p)?;
				let action: Action = match toml::from_str(content.as_str()) {
					Ok(a) => a,
					Err(_) => continue,
				};

				let key = action.trigger.get_inner().clone();
				m.insert(key, action);
			}
		}
	}

	Ok(m)
}
