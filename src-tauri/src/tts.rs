use std::sync::{LazyLock, nonpoison::RwLock};

use serde::{Deserialize, Serialize};
use specta::Type;
use tts::{Features, Tts, Voice};

use crate::{
	config::CONFIG,
	error::{Error, ErrorMsg, Result},
};

#[derive(Debug, Deserialize, Serialize, Type)]
pub enum TtsBackend {
	System,
}

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct VoiceData {
	pub language: String,
	pub name: String,
}

pub struct TtsData {
	tts: tts::Tts,
	voices: Vec<Voice>,
	selected_voice: usize,
	is_speaking: bool,
}

fn init_tts_data() -> Result<TtsData, Error> {
	let tts = Tts::default()?;

	let Features {
		utterance_callbacks,
		voice,
		..
	} = tts.supported_features();

	if utterance_callbacks {
		tts.on_utterance_begin(Some(Box::new(|_| {
			if let Some(tts_data) = TTS_DATA.write().as_mut() {
				tts_data.is_speaking = true;
			}
		})))?;
		tts.on_utterance_end(Some(Box::new(|_| {
			if let Some(tts_data) = TTS_DATA.write().as_mut() {
				tts_data.is_speaking = false;
			}
		})))?;
		tts.on_utterance_stop(Some(Box::new(|_| {
			if let Some(tts_data) = TTS_DATA.write().as_mut() {
				tts_data.is_speaking = false;
			}
		})))?;
	} else {
		tracing::warn!("Utterance-Callbacks are not supported!");
	}

	let voices: Vec<Voice> = if voice {
		match tts.voices() {
			Ok(v) => v,
			Err(e) => {
				tracing::warn!("Error getting tts voices {e}");
				Vec::new()
			}
		}
	} else {
		tracing::warn!("Voices not available!");
		Vec::new()
	};

	let mut voice_index = 0;
	if let Some(tts_cfg) = CONFIG.read().tts.as_ref()
		&& let Some(voice) = &tts_cfg.voice
	{
		let found_index = voices
			.iter()
			.enumerate()
			.find(|(_, v)| v.language() == voice.language && v.name() == voice.name)
			.map(|(i, _)| i);

		if let Some(i) = found_index {
			voice_index = i;
		}
	}

	Ok(TtsData {
		tts,
		voices,
		selected_voice: voice_index,
		is_speaking: false,
	})
}

static TTS_DATA: LazyLock<RwLock<Option<TtsData>>> = LazyLock::new(|| {
	RwLock::new(match init_tts_data() {
		Ok(t) => Some(t),
		Err(e) => {
			tracing::error!("Error initializing TTS {e}");
			None
		}
	})
});

pub fn get_active() -> Option<VoiceData> {
	let lock = TTS_DATA.read();
	let tts_data = lock.as_ref()?;
	let voice = tts_data.voices.get(tts_data.selected_voice)?;
	Some(VoiceData {
		language: voice.language().to_string(),
		name: voice.name(),
	})
}

pub fn get_voices() -> Vec<VoiceData> {
	match TTS_DATA.read().as_ref() {
		None => Vec::new(),
		Some(tts_data) => tts_data
			.voices
			.iter()
			.map(|v| VoiceData {
				language: v.language().to_string(),
				name: v.name(),
			})
			.collect(),
	}
}

pub fn set_active_voice(voice: &VoiceData) -> Result<(), Error> {
	let mut tts_data = TTS_DATA.write();

	let tts_data = match tts_data.as_mut() {
		None => return Ok(()),
		Some(t) => t,
	};

	let (index, voice) = match tts_data
		.voices
		.iter()
		.enumerate()
		.find(|(_, v)| v.language() == voice.language && v.name() == voice.name)
	{
		Some(v) => v,
		None => return Err("".into()),
	};

	tts_data.tts.set_voice(voice)?;
	tts_data.selected_voice = index;

	Ok(())
}

pub fn speak(s: String, voice_overwrite: Option<VoiceData>) -> Result<(), Error> {
	if s.is_empty() {
		return Err(Error::from_str("Cannot say empty message", ErrorMsg::Tts));
	}

	if let Some(tts_data) = TTS_DATA.write().as_mut() {
		if let Some(overwrite) = &voice_overwrite
			&& let Some(voice) = tts_data
				.voices
				.iter()
				.find(|v| v.language() == overwrite.language && v.name() == overwrite.name)
		{
			tts_data.tts.set_voice(voice)?;
		}

		tts_data.tts.speak(s, false)?;

		if voice_overwrite.is_some()
			&& let Some(voice) = tts_data.voices.get(tts_data.selected_voice)
		{
			tts_data.tts.set_voice(voice)?;
		}
	}

	Ok(())
}

// pub fn test() -> Result<()> {
// 	let mut tts = Tts::default()?;
// 	let Features {
// 		utterance_callbacks,
// 		..
// 	} = tts.supported_features();
// 	if utterance_callbacks {
// 		tts.on_utterance_begin(Some(Box::new(|utterance| {
// 			println!("Started speaking {:?}", utterance)
// 		})))?;
// 		tts.on_utterance_end(Some(Box::new(|utterance| {
// 			println!("Finished speaking {:?}", utterance)
// 		})))?;
// 		tts.on_utterance_stop(Some(Box::new(|utterance| {
// 			println!("Stopped speaking {:?}", utterance)
// 		})))?;
// 	}
// 	let Features { is_speaking, .. } = tts.supported_features();
// 	if is_speaking {
// 		println!("Are we speaking? {}", tts.is_speaking()?);
// 	}
// 	tts.speak("Hello, world.", false)?;
// 	let Features { rate, .. } = tts.supported_features();
// 	if rate {
// 		let original_rate = tts.get_rate()?;
// 		tts.speak(format!("Current rate: {}", original_rate), false)?;
// 		tts.set_rate(tts.max_rate())?;
// 		tts.speak("This is very fast.", false)?;
// 		tts.set_rate(tts.min_rate())?;
// 		tts.speak("This is very slow.", false)?;
// 		tts.set_rate(tts.normal_rate())?;
// 		tts.speak("This is the normal rate.", false)?;
// 		tts.set_rate(original_rate)?;
// 	}
// 	let Features { pitch, .. } = tts.supported_features();
// 	if pitch {
// 		let original_pitch = tts.get_pitch()?;
// 		tts.set_pitch(tts.max_pitch())?;
// 		tts.speak("This is high-pitch.", false)?;
// 		tts.set_pitch(tts.min_pitch())?;
// 		tts.speak("This is low pitch.", false)?;
// 		tts.set_pitch(tts.normal_pitch())?;
// 		tts.speak("This is normal pitch.", false)?;
// 		tts.set_pitch(original_pitch)?;
// 	}
// 	let Features { volume, .. } = tts.supported_features();
// 	if volume {
// 		let original_volume = tts.get_volume()?;
// 		tts.set_volume(tts.max_volume())?;
// 		tts.speak("This is loud!", false)?;
// 		tts.set_volume(tts.min_volume())?;
// 		tts.speak("This is quiet.", false)?;
// 		tts.set_volume(tts.normal_volume())?;
// 		tts.speak("This is normal volume.", false)?;
// 		tts.set_volume(original_volume)?;
// 	}
// 	let Features { voice, .. } = tts.supported_features();
// 	if voice {
// 		let voices = tts.voices()?;
// 		println!("Available voices:\n===");
// 		for v in &voices {
// 			println!("{:?}", v);
// 		}
// 		let Features { get_voice, .. } = tts.supported_features();
// 		let original_voice = if get_voice { tts.voice()? } else { None };
// 		for v in &voices {
// 			tts.set_voice(v)?;
// 			tts.speak(format!("This is {}.", v.name()), false)?;
// 		}
// 		if let Some(original_voice) = original_voice {
// 			tts.set_voice(&original_voice)?;
// 		}
// 	}
// 	tts.speak("Goodbye.", false)?;
// 	let mut _input = String::new();
// 	// The below is only needed to make the example run on MacOS because there is no NSRunLoop in this context.
// 	// It shouldn't be needed in an app or game that almost certainly has one already.
// 	#[cfg(target_os = "macos")]
// 	{
// 		let run_loop = unsafe { objc2_foundation::NSRunLoop::currentRunLoop() };
// 		unsafe { run_loop.run() };
// 	}
// 	io::stdin().read_line(&mut _input)?;
// 	Ok(())
// }
