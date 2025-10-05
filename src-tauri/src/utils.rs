use std::{
	env::current_dir,
	path::PathBuf,
	sync::LazyLock,
	time::{SystemTime, UNIX_EPOCH},
};

pub static NAME: &str = env!("CARGO_PKG_NAME");

pub static NAME_CAPITALIZED: &str = "Bärbot";

pub static CFG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	match dirs::config_dir() {
		Some(mut p) => {
			p.push(NAME);
			p
		}
		None => current_dir().expect("Couldn't get current dir"), // we should never even hit this so expect should be fine
	}
});

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
