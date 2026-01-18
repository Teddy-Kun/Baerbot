use crate::tts::TtsSystem;

pub struct TtsConfig {}

impl TtsSystem for TtsConfig {
	fn get_active_voice(&self) -> Option<super::VoiceData> {
		todo!()
	}

	fn get_voices(&self) -> Vec<super::VoiceData> {
		todo!()
	}

	fn set_active_voice(&mut self, voice: &super::VoiceData) -> Result<(), crate::error::Error> {
		todo!()
	}

	fn speak(
		&mut self,
		s: String,
		voice_overwrite: Option<super::VoiceData>,
	) -> Result<(), crate::error::Error> {
		todo!()
	}
}
