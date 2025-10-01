use std::{
	borrow::{Borrow, Cow},
	collections::HashMap,
	fs::{self, OpenOptions, create_dir_all, read_dir, remove_file},
	io::Write,
	ops::Deref,
	sync::{
		Arc, LazyLock,
		atomic::{AtomicU64, Ordering},
	},
	time::{SystemTime, UNIX_EPOCH},
};

use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::async_runtime::RwLock;

use crate::{error::Error, twitch::counter::TwitchCounter, utils::CFG_DIR_PATH};

static ACTION_TABLE: LazyLock<RwLock<HashMap<ArcStr, Action>>> = LazyLock::new(|| {
	let m = init_map().unwrap_or_default();
	RwLock::new(m)
});

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

impl From<ArcStr> for Arc<str> {
	fn from(value: ArcStr) -> Self {
		value.0.clone()
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

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq, PartialOrd, Ord)]
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

static FIND_RANGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\{\d+\.\.\d+\})+").unwrap());
pub fn process_reply(s: &str) -> Cow<'_, str> {
	let mut rng = rand::rng();

	FIND_RANGE.replace_all(s, |caps: &regex::Captures| {
		tracing::debug!("caps: {:?}", caps);

		let mut s = caps[0].trim_prefix('{').trim_suffix('}').split("..");

		let start: i64 = s.next().unwrap().parse().unwrap();
		let end: i64 = s.next().unwrap().parse().unwrap();
		let num = rng.random_range(start..=end);
		num.to_string()
	})
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Action {
	pub trigger: Trigger,
	pub exec: Exec,
	#[serde(skip)]
	pub last_used: Arc<AtomicU64>,
}

impl Action {
	pub fn allow_use(&self) -> bool {
		let now = SystemTime::now();
		let since = now
			.duration_since(UNIX_EPOCH)
			.map(|res| (res.as_millis() as u64) - 5000) // 5 second cooldown; TODO: make configurable per Action
			.unwrap_or(0);

		// load the current value atomically
		let mut current = self.last_used.load(Ordering::Relaxed);

		loop {
			// if since is smaller then current just return false
			if since <= current {
				return false;
			}

			// if not try and set the new value
			// this will only return ok, if between the load and this operation, the value wasn't overwritten
			match self.last_used.compare_exchange(
				current,
				since,
				Ordering::SeqCst,
				Ordering::Relaxed,
			) {
				Ok(_) => return true,        // we successfully set the new value
				Err(prev) => current = prev, // if it was overwritten, check again next iteration, where we will most likely return false
			}
		}
	}
}

impl PartialEq for Action {
	fn eq(&self, other: &Self) -> bool {
		self.trigger.eq(&other.trigger)
	}
}

impl PartialOrd for Action {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Eq for Action {}

impl Ord for Action {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.trigger.cmp(&other.trigger)
	}
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
	match table.remove(key) {
		Some(_) => tracing::debug!("removed action {key}"),
		None => tracing::warn!("Action {key} was not found, nothing removed"),
	}
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
	let mut v: Vec<Action> = table.values().cloned().collect();
	v.sort_unstable();
	v
}

async fn save_actions_inner(table: &HashMap<ArcStr, Action>) -> Result<(), Error> {
	let p = CFG_DIR_PATH.join("actions");

	tracing::debug!("actions dir {p:?}");

	create_dir_all(&p)?;

	let errs = table.values().filter_map(|action| {
		let mut p = p.clone();

		p.push(format!("{}.toml", action.trigger.deref()));

		tracing::info!("action path {p:?}");

		let s = match toml::to_string_pretty(&action) {
			Ok(s) => s,
			Err(e) => {
				let e: Error = e.into();
				return Some(e);
			}
		};

		let mut f = match OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(p)
		{
			Ok(f) => f,
			Err(e) => {
				let e: Error = e.into();
				return Some(e);
			}
		};

		match f.write_all(s.as_bytes()) {
			Ok(_) => None,
			Err(e) => {
				let e: Error = e.into();
				Some(e)
			}
		}
	});

	errs.for_each(|e| {
		tracing::warn!("Couldn't save action file {e}");
	});

	Ok(())
}

fn delete_action_from_fs(key: &str) -> Result<(), Error> {
	let path = CFG_DIR_PATH.join("actions");

	let target_name = format!("{key}.toml");
	tracing::debug!("Trying to delete {target_name}");

	let count = read_dir(&path)?
		.filter_map(Result::ok)
		.map(|entry| entry.path())
		.filter(|path| {
			path.is_file()
				&& path
					.file_name()
					.and_then(|n| n.to_str())
					.map(|n| n.eq_ignore_ascii_case(&target_name))
					.unwrap_or(false)
		})
		.inspect(|path| {
			tracing::debug!("Found {target_name}, removing...");
			if let Err(e) = remove_file(path) {
				tracing::error!("Error deleting action from fs: {e}")
			}
		})
		.count();

	if count == 0 {
		tracing::warn!("File `{target_name}` not found in {:?}", path);
	}

	Ok(())
}

fn init_map() -> Result<HashMap<ArcStr, Action>, Error> {
	let mut m = HashMap::new();

	let path = CFG_DIR_PATH.join("actions");

	read_dir(&path)?
		.filter_map(Result::ok)
		.map(|entry| entry.path())
		.filter(|path| {
			path.is_file()
				&& path
					.extension()
					.and_then(|extension| extension.to_str())
					.map(|extension| extension.eq_ignore_ascii_case("toml"))
					.unwrap_or(false)
		})
		.filter_map(|path| {
			let content = fs::read_to_string(&path).ok()?;
			toml::from_str::<Action>(content.as_str()).ok()
		})
		.for_each(|action| {
			let key = action.trigger.get_inner().clone();
			m.insert(key, action);
		});

	Ok(m)
}
