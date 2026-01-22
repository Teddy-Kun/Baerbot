use std::{
	env::current_dir,
	ops::Deref,
	path::PathBuf,
	sync::LazyLock,
	time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use specta::Type;

pub static NAME: &str = env!("CARGO_PKG_NAME");
pub static NAME_CAPITALIZED: &str = "Beanybot";

pub static CFG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	match dirs::config_dir() {
		Some(mut p) => {
			p.push(NAME);
			p
		}
		None => current_dir().expect("Couldn't get current dir"), // we should never even hit this so expect should be fine
	}
});

pub static ACTION_DIR: LazyLock<PathBuf> = LazyLock::new(|| CFG_DIR_PATH.join("actions"));

pub fn get_unix() -> u64 {
	let now = SystemTime::now();
	now.duration_since(UNIX_EPOCH)
		.map(|res| res.as_secs())
		.unwrap_or(0)
}

pub fn get_unix_milli() -> u64 {
	let now = SystemTime::now();
	now.duration_since(UNIX_EPOCH)
		.map(|res| res.as_millis() as u64)
		.unwrap_or(0)
}

/// Could be either an owned String, or a &'static str.
/// Using this prevents unnecessarily copying the &'static str.
/// Cow<'a, str> could be used instead but this prevents modification and thus reallocation.
/// Also its potentially very slightly, minimally more optimized.
#[derive(Debug)]
pub enum MaybeOwnedStr {
	String(String),
	Str(&'static str),
}

impl MaybeOwnedStr {
	#[inline]
	pub fn as_str(&self) -> &str {
		match self {
			Self::Str(s) => s,
			Self::String(s) => s.as_str(),
		}
	}
}

impl Serialize for MaybeOwnedStr {
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.as_str().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for MaybeOwnedStr {
	#[inline]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		Ok(Self::String(s))
	}
}

impl Type for MaybeOwnedStr {
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

impl From<String> for MaybeOwnedStr {
	#[inline]
	fn from(value: String) -> Self {
		MaybeOwnedStr::String(value)
	}
}

impl From<&'static str> for MaybeOwnedStr {
	#[inline]
	fn from(value: &'static str) -> Self {
		MaybeOwnedStr::Str(value)
	}
}

impl Deref for MaybeOwnedStr {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}
