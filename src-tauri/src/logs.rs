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

struct LogFile {
	name: String,
	date: i64,
}

impl PartialEq for LogFile {
	fn eq(&self, other: &Self) -> bool {
		self.date == other.date
	}
}

impl Eq for LogFile {}

impl PartialOrd for LogFile {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.date.partial_cmp(&other.date)
	}
}

impl Ord for LogFile {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.date.cmp(&other.date)
	}
}

static CURRENT_LOG_FILE: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
	let dir = read_dir(LOG_PATH.as_path()).ok()?;

	let p = dir
		.filter_map(Result::ok)
		.filter_map(|entry| {
			let filename = entry.file_name().into_string().ok()?;

			Some(LogFile {
				date: log_name_to_unix(filename.as_str())?,
				name: filename,
			})
		})
		.max()
		.map(|latest_file| LOG_PATH.join(latest_file.name))?;
	Some(p)
});

pub fn setup_logging() {
	let dbg = ARGS.debug;

	let lvl = match dbg {
		true => Level::DEBUG,
		false => Level::INFO,
	};

	let stdout = std::io::stdout.with_max_level(tracing::Level::DEBUG);
	let roller = tracing_appender::rolling::daily(LOG_PATH.as_path(), LOG_NAME.as_ref());

	let sub = tracing_subscriber::fmt()
		.with_writer(stdout.and(roller))
		.with_max_level(lvl)
		.finish();
	if let Err(err) = tracing::subscriber::set_global_default(sub) {
		eprintln!("Error setting up logger: {}", err)
	}
}

/// Converts a filename.yyyy-MM-dd into a unix timestamp.
/// Its not 100% accurate, since it uses averages for seconds per month/year, but its enough for sorting
fn log_name_to_unix(name: &str) -> Option<i64> {
	const SECONDS_PER_DAY: i64 = 60 * 60 * 24;
	const SECONDS_PER_MONTH: i64 = 2629800; // SECONDS_PER_DAY * 30.4375 || Average of days per month with February having 28.25 days
	const SECONDS_PER_YEAR: i64 = 31557600; // SECONDS_PER_DAY * 365.25

	// split the name into at max 3 parts
	let mut parts = name.splitn(3, '.');
	let filename = parts.next()?; // get filename & return on empty which should be impossible
	let maybe_date = parts.next()?; // get date & return when we have only 1 element
	let third = parts.next();

	// since proper log filenames should be filename.date any third argument would mean its not a log file we created so we can skip right here
	if third.is_some() {
		return None;
	}

	if filename != LOG_NAME.as_ref() {
		// not a log we created, skip
		return None;
	}

	let mut split_date = maybe_date.splitn(4, '-');

	let year = split_date.next()?;
	let month = split_date.next()?;
	let day = split_date.next()?;
	let fourth = split_date.next();

	if fourth.is_some() {
		return None;
	}

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

	Some(unix)
}

pub fn get_latest_log() -> Result<Option<String>, Error> {
	let p = CURRENT_LOG_FILE.as_ref();
	match p {
		None => Ok(None),
		Some(p) => {
			let res = read_to_string(p)?;
			Ok(Some(res))
		}
	}
}
