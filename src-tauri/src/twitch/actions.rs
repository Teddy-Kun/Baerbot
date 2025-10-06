use std::{
	borrow::{Borrow, Cow},
	fs::{self, OpenOptions, create_dir_all, read_dir, remove_file},
	io::Write,
	ops::Deref,
	sync::{
		Arc, LazyLock,
		atomic::{AtomicU64, Ordering},
	},
	time::{SystemTime, UNIX_EPOCH},
};

use dashmap::DashMap;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
	error::Error,
	twitch::{TWITCH_CLIENT, counter::TwitchCounter},
	utils::ACTION_DIR,
};

static ACTION_TABLE: LazyLock<DashMap<ArcStr, Action>> =
	LazyLock::new(|| init_map().unwrap_or_default());

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
pub enum ExecTarget {
	None,
	User,
	Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum Exec {
	ChatMsg(ArcStr),
	Reply(ArcStr),
	Counter(TwitchCounter),
	Timeout(ExecTarget, u32),
	Ban(ExecTarget),
	Chance(f64, Box<Exec>, Box<Exec>),
}

impl Exec {
	pub async fn exec(&self, user: &str, prompt: Option<&str>) -> Option<()> {
		let tw_client = TWITCH_CLIENT.read().await;
		let username = match tw_client.get_username() {
			None => {
				tracing::error!("Username gone");
				return None;
			}
			Some(u) => u,
		};
		let client = tw_client.chat_client.clone();

		match self {
			Exec::ChatMsg(msg) => match client {
				None => {
					tracing::error!("Chat client not set up");
					None
				}
				Some(client) => {
					if let Err(e) = client
						.say(username, process_reply(msg.as_ref()).to_string())
						.await
					{
						tracing::error!("Couldn't send chat msg: {e}");
						None
					} else {
						Some(())
					}
				}
			},
			Exec::Timeout(target, timeout) => {
				let target_user = match target {
					ExecTarget::None => return None,
					ExecTarget::User => user,
					ExecTarget::Other => prompt?,
				};

				tw_client.ban_user(target_user, "", Some(*timeout)).await
			}
			Exec::Ban(target) => {
				let target_user = match target {
					ExecTarget::None => return None,
					ExecTarget::User => user,
					ExecTarget::Other => prompt?,
				};

				tw_client.ban_user(target_user, "", None).await
			}
			Exec::Chance(chance, opt1, opt2) => {
				drop(tw_client); // freeing the lock is required here
				let random_f: f64;
				{
					// scope here because rng does not implement send
					let mut rng = rand::rng();
					random_f = rng.random_range(0.0..1.0);
				}
				if random_f < *chance {
					Box::pin(opt1.exec(user, prompt)).await
				} else {
					Box::pin(opt2.exec(user, prompt)).await
				}
			}
			e => todo!("{e:?}"),
		}
	}
}

static FIND_RANGE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\{\d+\.\.\d+\})+").unwrap());
fn process_reply(s: &str) -> Cow<'_, str> {
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
	let a = ACTION_TABLE.get(key)?;
	Some(a.value().clone())
}

pub async fn add_action(action: Action) {
	_ = ACTION_TABLE.insert(action.trigger.get_inner().clone(), action);

	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = save_actions().await {
		tracing::error!("Error saving actions: {e}")
	};
}

pub async fn drop_action(key: &str) {
	match ACTION_TABLE.remove(key) {
		Some(_) => tracing::debug!("removed action {key}"),
		None => tracing::warn!("Action {key} was not found, nothing removed"),
	}

	// we keep the writing lock to ensure no other writes interrupt us
	if let Err(e) = delete_action_from_fs(key) {
		tracing::error!("Error deleting action from fs: {e}")
	};
}

pub async fn get_all_actions() -> Vec<Action> {
	let mut v: Vec<Action> = ACTION_TABLE
		.iter()
		.map(|inner| inner.value().clone())
		.collect();
	v.sort_unstable();
	v
}

pub async fn save_actions() -> Result<(), Error> {
	create_dir_all(ACTION_DIR.as_path())?;

	let errs = ACTION_TABLE.iter().filter_map(|inner| {
		let action = inner.value();
		save_action(action).err()
	});

	errs.for_each(|e| {
		tracing::warn!("Couldn't save action file {e}");
	});

	Ok(())
}

fn save_action(action: &Action) -> Result<(), Error> {
	let p = ACTION_DIR.join(format!("{}.toml", action.trigger.deref()));

	tracing::info!("action path {p:?}");

	let s = toml::to_string_pretty(action)?;

	let mut f = OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(p)?;

	f.write_all(s.as_bytes())?;
	Ok(())
}

fn delete_action_from_fs(key: &str) -> Result<(), Error> {
	let target_name = format!("{key}.toml");
	tracing::debug!("Trying to delete {target_name}");

	let count = read_dir(ACTION_DIR.as_path())?
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
		tracing::warn!(
			"File `{target_name}` not found in {:?}",
			ACTION_DIR.as_path()
		);
	}

	Ok(())
}

fn init_map() -> Result<DashMap<ArcStr, Action>, Error> {
	let m = DashMap::new();

	read_dir(ACTION_DIR.as_path())?
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
