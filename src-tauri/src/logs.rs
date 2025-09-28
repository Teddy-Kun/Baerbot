use std::{
	fs::{read_dir, read_to_string},
	path::PathBuf,
	sync::LazyLock,
};

use tedbot_lib::{
	config::ARGS,
	error::Error,
	utils::{CFG_DIR_PATH, NAME},
};
use tracing::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;

static LOG_PATH: LazyLock<PathBuf> = LazyLock::new(|| CFG_DIR_PATH.join("logs"));

static LOG_NAME: LazyLock<Box<str>> = LazyLock::new(|| {
	let mut log_name = NAME.clone();
	log_name += "_log";
	log_name.into_boxed_str()
});

pub fn setup_logging() {
	let dbg = ARGS.debug;

	let lvl = match dbg {
		true => Level::DEBUG,
		false => Level::INFO,
	};

	let stdout = std::io::stdout.with_max_level(tracing::Level::DEBUG);
	let roller = tracing_appender::rolling::hourly(LOG_PATH.as_path(), LOG_NAME.as_ref());

	let sub = tracing_subscriber::fmt()
		.with_writer(stdout.and(roller))
		.with_max_level(lvl)
		.finish();
	if let Err(err) = tracing::subscriber::set_global_default(sub) {
		eprintln!("Error setting up logger: {}", err)
	}
}

const SECONDS_PER_HOUR: i64 = 60 * 60;
const SECONDS_PER_DAY: i64 = SECONDS_PER_HOUR * 24;
const SECONDS_PER_MONTH: i64 = 2629800; // SECONDS_PER_DAY * 30.4375 || Average of days per month with February having 28.25 days
const SECONDS_PER_YEAR: i64 = 31557600; // SECONDS_PER_DAY * 365.25

/// Converts a filename.yyyy-MM-dd-HH into a unix timestamp.
/// Its not 100% accurate, since it uses averages for seconds per month/year, but its enough for sorting
fn log_name_to_unix(name: &str) -> Option<i64> {
	let v: Vec<&str> = name.split('.').collect();
	if v.len() != 2 {
		return None;
	}

	if *unsafe { v.get_unchecked(0) } != LOG_NAME.as_ref() {
		return None;
	}

	// guaranteed safe, since v.len is exactly 2 here
	let maybe_date = *unsafe { v.get_unchecked(1) };
	let split_date: Vec<&str> = maybe_date.split('-').collect();

	if split_date.len() != 4 {
		return None;
	}

	let year = *unsafe { split_date.get_unchecked(0) };
	let month = *unsafe { split_date.get_unchecked(1) };
	let day = *unsafe { split_date.get_unchecked(2) };
	let hour = *unsafe { split_date.get_unchecked(3) };

	let mut unix: i64;
	match year.parse::<i64>() {
		Ok(y) => unix = (y - 1970) * SECONDS_PER_YEAR,
		Err(_) => return None,
	}

	match month.parse::<i64>() {
		Ok(m) => unix += m * SECONDS_PER_MONTH,
		Err(_) => return None,
	}

	match day.parse::<i64>() {
		Ok(d) => unix += d * SECONDS_PER_DAY,
		Err(_) => return None,
	}

	match hour.parse::<i64>() {
		Ok(h) => unix += h * SECONDS_PER_HOUR,
		Err(_) => return None,
	}

	Some(unix)
}

pub fn get_latest_log_file() -> Result<Option<String>, Error> {
	let dir = read_dir(LOG_PATH.as_path())?;

	tracing::debug!("GOT DIR");

	struct LogFile {
		name: String,
		date: i64,
	}
	let mut latest_file: Option<LogFile> = None;

	for entry in dir {
		let entry = match entry {
			Ok(e) => e,
			Err(_) => continue,
		};

		let name = match entry.file_name().into_string() {
			Ok(n) => n,
			Err(_) => continue,
		};

		let unix = match log_name_to_unix(name.as_str()) {
			Some(u) => u,
			None => continue,
		};

		match &latest_file {
			None => {
				latest_file = Some(LogFile { name, date: unix });
			}
			Some(v) => {
				if v.date < unix {
					latest_file = Some(LogFile { name, date: unix });
				}
			}
		}
	}

	match latest_file {
		None => Ok(None),
		Some(f) => {
			let f_path = LOG_PATH.join(f.name);
			let res = read_to_string(f_path)?;
			Ok(Some(res))
		}
	}
}
