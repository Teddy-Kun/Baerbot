use std::{
	collections::HashMap,
	fs::OpenOptions,
	io::Write,
	sync::{Arc, LazyLock},
};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{dirs::CFG_DIR_PATH, error::Error};

static ACTION_TABLE: LazyLock<RwLock<HashMap<Arc<str>, Action>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
	Command(Arc<str>),
	Redeem(Arc<str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Exec {
	ChatMsg(Arc<str>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
	trigger: Trigger,
	exec: Exec,
}

pub async fn get_action(key: &str) -> Option<Action> {
	let table = ACTION_TABLE.read().await;
	table.get(key).cloned()
}

pub async fn add_action(key: Arc<str>, action: Action) {
	let mut table = ACTION_TABLE.write().await;
	table.insert(key, action);
	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = save_actions(&table) {
		tracing::error!("Error saving actions: {e}")
	};
}

pub async fn drop_action(key: &str) {
	let mut table = ACTION_TABLE.write().await;
	table.remove(key);
	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = save_actions(&table) {
		tracing::error!("Error saving actions: {e}")
	};
}

fn save_actions(table: &HashMap<Arc<str>, Action>) -> Result<(), Error> {
	let v: Vec<&Action> = table.values().collect();

	let s = toml::to_string_pretty(&v)?;
	drop(v);

	let mut p = CFG_DIR_PATH.clone();
	p.push("actions.toml");
	let mut f = OpenOptions::new().write(true).truncate(true).open(p)?;
	f.write_all(s.as_bytes())?;

	Ok(())
}
