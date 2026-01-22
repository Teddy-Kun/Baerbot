use tts::{Features, Tts, UtteranceId, Voice};

pub struct TtsConfig {
	voices: Vec<Voice>,
	tts: Tts,
	selected_voice: Option<usize>,
}

use crate::{
	config::CONFIG,
	error::{Error, ErrorMsg},
	tts::{TtsSystem, VoiceData},
};

pub fn init_tts_config(
	begin: Option<Box<dyn FnMut(UtteranceId)>>,
	end: Option<Box<dyn FnMut(UtteranceId)>>,
	stop: Option<Box<dyn FnMut(UtteranceId)>>,
) -> Result<TtsConfig, Error> {
	let tts = Tts::default()?;

	let Features {
		utterance_callbacks,
		voice,
		..
	} = tts.supported_features();

	if utterance_callbacks {
		tts.on_utterance_begin(begin)?;
		tts.on_utterance_end(end)?;
		tts.on_utterance_stop(stop)?;
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

	let mut voice_index = None;
	if let Some(tts_cfg) = CONFIG.read().tts.as_ref()
		&& let Some(voice) = &tts_cfg.voice
	{
		let found_index = voices
			.iter()
			.enumerate()
			.find(|(_, v)| {
				v.language().as_str() == voice.language.as_str()
					&& v.name().as_str() == voice.name.as_str()
			})
			.map(|(i, _)| i);

		voice_index = found_index;
	}

	Ok(TtsConfig {
		tts,
		voices,
		selected_voice: voice_index,
	})
}

impl TtsSystem for TtsConfig {
	fn get_active_voice(&self) -> Option<VoiceData> {
		let voice = self.voices.get(self.selected_voice?)?;
		Some(VoiceData {
			language: voice.language().to_string().into(),
			name: voice.name().into(),
		})
	}

	fn get_voices(&self) -> Vec<VoiceData> {
		self.voices
			.iter()
			.map(|v| VoiceData {
				language: v.language().to_string().into(),
				name: v.name().into(),
			})
			.collect()
	}

	fn set_active_voice(&mut self, voice: &VoiceData) -> Result<(), Error> {
		let (index, voice) = match self.voices.iter().enumerate().find(|(_, v)| {
			v.language().as_str() == voice.language.as_str()
				&& v.name().as_str() == voice.name.as_str()
		}) {
			Some(v) => v,
			None => {
				return Err(Error::from_str(
					"Couldn't set active voice: Voice not found",
					ErrorMsg::Tts,
				));
			}
		};

		self.tts.set_voice(voice)?;
		self.selected_voice = Some(index);

		Ok(())
	}

	fn speak(&mut self, s: String, voice_overwrite: Option<VoiceData>) -> Result<(), Error> {
		if s.is_empty() {
			return Err(Error::from_str("Cannot say empty message", ErrorMsg::Tts));
		}

		let mut overwrote = false;
		if let Some(overwrite) = voice_overwrite {
			match self.voices.iter().find(|v| {
				v.language().as_str() == overwrite.language.as_str()
					&& v.name().as_str() == overwrite.name.as_str()
			}) {
				None => tracing::warn!("Voice overwrite not found"),
				Some(voice) => {
					self.tts.set_voice(voice)?;
					overwrote = true;
				}
			}
		}

		self.tts.speak(s, false)?;

		if overwrote && let Some(selected) = self.selected_voice {
			if let Some(voice) = self.voices.get(selected) {
				self.tts.set_voice(voice)?;
			} else {
				tracing::warn!("Couldn't reset to default voice: voice not found?");
			}
		}

		Ok(())
	}
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
