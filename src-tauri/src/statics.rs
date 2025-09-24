use std::{env::current_dir, path::PathBuf, sync::LazyLock};

pub static NAME: LazyLock<String> = LazyLock::new(|| String::from(env!("CARGO_PKG_NAME")));

pub static NAME_CAPITALIZED: LazyLock<String> = LazyLock::new(|| {
	let mut c = NAME.chars();
	match c.next() {
		Some(u) => u.to_uppercase().chain(c).collect(),
		None => String::new(),
	}
});

pub static CFG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	match dirs::config_dir() {
		Some(mut p) => {
			p.push(env!("CARGO_PKG_NAME"));
			p
		}
		None => current_dir().expect("Couldn't get current dir"), // we should never even hit this so expect should be fine
	}
});
