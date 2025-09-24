use std::{env::current_dir, path::PathBuf, sync::LazyLock};

pub static CFG_DIR_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
	match dirs::config_dir() {
		Some(mut p) => {
			p.push(env!("CARGO_CRATE_NAME"));
			p
		}
		None => current_dir().expect("Couldn't get current dir"), // we should never even hit this so expect should be fine
	}
});
