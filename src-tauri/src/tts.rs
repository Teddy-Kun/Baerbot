use std::sync::{LazyLock, nonpoison::Mutex};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::error::Error;

mod piper;
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
	pub language: String,
	pub name: String,
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

// TODO: init
static TTS_DATA: LazyLock<Mutex<Option<TtsData>>> = LazyLock::new(|| Mutex::new(None));

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
