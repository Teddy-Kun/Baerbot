use std::{
	cmp::Ordering,
	sync::{LazyLock, nonpoison::Mutex},
};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{error::Error, utils::MaybeOwnedStr};

pub mod piper;
mod system;

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Type)]
pub enum TtsBackend {
	System,
	Piper,
}

enum TtsBackendCfg {
	System(system::TtsConfig),
	Piper(piper::TtsConfig),
}

impl TtsBackendCfg {
	fn as_trait(&self) -> &dyn TtsSystem {
		match self {
			TtsBackendCfg::System(cfg) => cfg,
			TtsBackendCfg::Piper(cfg) => cfg,
		}
	}

	fn as_trait_mut(&mut self) -> &mut dyn TtsSystem {
		match self {
			TtsBackendCfg::System(cfg) => cfg,
			TtsBackendCfg::Piper(cfg) => cfg,
		}
	}
}

impl From<&TtsBackendCfg> for TtsBackend {
	fn from(value: &TtsBackendCfg) -> Self {
		match value {
			TtsBackendCfg::System(_) => TtsBackend::System,
			TtsBackendCfg::Piper(_) => TtsBackend::Piper,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct VoiceData {
	pub language: MaybeOwnedStr,
	pub name: MaybeOwnedStr,
}

impl Eq for VoiceData {}

impl PartialEq for VoiceData {
	fn eq(&self, other: &Self) -> bool {
		self.language.as_str() == other.language.as_str()
			&& self.name.as_str() == other.name.as_str()
	}
}

impl Ord for VoiceData {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let lang = self.language.as_str().cmp(other.language.as_str());
		if lang != Ordering::Equal {
			return lang;
		}

		self.name.as_str().cmp(other.name.as_str())
	}
}

impl PartialOrd for VoiceData {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

pub struct TtsData {
	cfg: TtsBackendCfg,
	is_speaking: bool,
}

pub trait TtsSystem {
	fn get_active_voice(&self) -> Option<VoiceData>;
	fn get_voices(&self) -> Vec<VoiceData>;
	fn set_active_voice(&mut self, voice: &VoiceData) -> Result<(), Error>;
	fn speak(&mut self, s: String, voice_overwrite: Option<VoiceData>) -> Result<(), Error>;
}

// TODO: init piper
static TTS_DATA: LazyLock<Mutex<Option<TtsData>>> = LazyLock::new(|| {
	Mutex::new(match system::init_tts_config(None, None, None) {
		Ok(t) => Some(TtsData {
			cfg: TtsBackendCfg::System(t),
			is_speaking: false,
		}),
		Err(e) => {
			tracing::error!("Couldn't set up system tts: {e}");
			None
		}
	})
});

pub fn get_active_voice() -> Option<VoiceData> {
	TTS_DATA.lock().as_ref()?.cfg.as_trait().get_active_voice()
}

pub fn get_voices() -> Vec<VoiceData> {
	let lock = TTS_DATA.lock();
	match lock.as_ref() {
		None => Vec::new(),
		Some(data) => data.cfg.as_trait().get_voices(),
	}
}

pub fn set_active_voice(voice: &VoiceData) -> Result<(), Error> {
	let mut lock = TTS_DATA.lock();
	match lock.as_mut() {
		None => Ok(()),
		Some(data) => data.cfg.as_trait_mut().set_active_voice(voice),
	}
}

pub fn speak(s: String, voice_overwrite: Option<VoiceData>) -> Result<(), Error> {
	let mut lock = TTS_DATA.lock();
	match lock.as_mut() {
		None => Ok(()),
		Some(data) => data.cfg.as_trait_mut().speak(s, voice_overwrite),
	}
}

// TODO: remove and think of a better API
pub fn activate_piper() {
	let mut data = TTS_DATA.lock();
	*data = Some(TtsData {
		cfg: TtsBackendCfg::Piper(piper::TtsConfig {}),
		is_speaking: false,
	});
}
