use std::fs::File;

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

#[derive(Debug, Clone, Copy)]
pub struct PiperVoiceUrls<'p> {
	example: &'p str,
	onnx: &'p str,
	json: &'p str,
}

impl<'p> PiperVoiceUrls<'p> {
	pub fn play_sample() {
		todo!()
	}

	pub fn download() {
		todo!()
	}
}

// src: https://github.com/Elleo/pied/blob/94f9024201447e6b40ecd0c73bb2837fed3caf54/lib/voice_data.dart#L25
pub static PIPER_VOICES: phf::Map<&'static str, phf::Map<&'static str, PiperVoiceUrls<'static>>> = phf::phf_map! {
	"ar_JO" => phf::phf_map! {
		"Kareem - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/low/ar_JO-kareem-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/low/ar_JO-kareem-low.onnx.json?download=true"
		},
		"Kareem - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/medium/ar_JO-kareem-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ar/ar_JO/kareem/medium/ar_JO-kareem-medium.onnx.json?download=true"
		}
	},
	"ca_ES" => phf::phf_map! {
		"UPC Ona - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/x_low/ca_ES-upc_ona-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/x_low/ca_ES-upc_ona-x_low.onnx.json?download=true"
		},
		"Upc Ona - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/medium/ca_ES-upc_ona-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_ona/medium/ca_ES-upc_ona-medium.onnx.json?download=true"
		},
		"UPC Pau - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_pau/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_pau/x_low/ca_ES-upc_pau-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ca/ca_ES/upc_pau/x_low/ca_ES-upc_pau-x_low.onnx.json?download=true"
		}
	},
	"cs_CZ" => phf::phf_map! {
		"Jirka - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/low/cs_CZ-jirka-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/low/cs_CZ-jirka-low.onnx.json?download=true"
		},
		"Jirka - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/medium/cs_CZ-jirka-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cs/cs_CZ/jirka/medium/cs_CZ-jirka-medium.onnx.json?download=true"
		}
	},
	"cy_GB" => phf::phf_map! {
		"Bu Tts - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/bu_tts/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/bu_tts/medium/cy_GB-bu_tts-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/bu_tts/medium/cy_GB-bu_tts-medium.onnx.json?download=true"
		},
		"Gwryw Gogleddol - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/gwryw_gogleddol/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/gwryw_gogleddol/medium/cy_GB-gwryw_gogleddol-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/cy/cy_GB/gwryw_gogleddol/medium/cy_GB-gwryw_gogleddol-medium.onnx.json?download=true"
		}
	},
	"da_DK" => phf::phf_map! {
		"Talesyntese - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/da/da_DK/talesyntese/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/da/da_DK/talesyntese/medium/da_DK-talesyntese-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/da/da_DK/talesyntese/medium/da_DK-talesyntese-medium.onnx.json?download=true"
		}
	},
	"de_DE" => phf::phf_map! {
		"Eva K - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/eva_k/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/eva_k/x_low/de_DE-eva_k-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/eva_k/x_low/de_DE-eva_k-x_low.onnx.json?download=true"
		},
		"Karlsson - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/karlsson/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/karlsson/low/de_DE-karlsson-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/karlsson/low/de_DE-karlsson-low.onnx.json?download=true"
		},
		"Kerstin - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/kerstin/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/kerstin/low/de_DE-kerstin-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/kerstin/low/de_DE-kerstin-low.onnx.json?download=true"
		},
		"MLS - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/mls/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/mls/medium/de_DE-mls-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/mls/medium/de_DE-mls-medium.onnx.json?download=true"
		},
		"Pavoque - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/pavoque/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/pavoque/low/de_DE-pavoque-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/pavoque/low/de_DE-pavoque-low.onnx.json?download=true"
		},
		"Ramona - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/ramona/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/ramona/low/de_DE-ramona-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/ramona/low/de_DE-ramona-low.onnx.json?download=true"
		},
		"Thorsten - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/low/de_DE-thorsten-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/low/de_DE-thorsten-low.onnx.json?download=true"
		},
		"Thorsten - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/medium/de_DE-thorsten-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/medium/de_DE-thorsten-medium.onnx.json?download=true"
		},
		"Thorsten - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/high/de_DE-thorsten-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten/high/de_DE-thorsten-high.onnx.json?download=true"
		},
		"Thorsten Emotional - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten_emotional/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten_emotional/medium/de_DE-thorsten_emotional-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/de/de_DE/thorsten_emotional/medium/de_DE-thorsten_emotional-medium.onnx.json?download=true"
		}
	},
	"el_GR" => phf::phf_map! {
		"Rapunzelina - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/el/el_GR/rapunzelina/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/el/el_GR/rapunzelina/low/el_GR-rapunzelina-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/el/el_GR/rapunzelina/low/el_GR-rapunzelina-low.onnx.json?download=true"
		}
	},
	"en_GB" => phf::phf_map! {
		"Alan - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/low/en_GB-alan-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/low/en_GB-alan-low.onnx.json?download=true"
		},
		"Alan - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/medium/en_GB-alan-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alan/medium/en_GB-alan-medium.onnx.json?download=true"
		},
		"Alba - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alba/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alba/medium/en_GB-alba-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/alba/medium/en_GB-alba-medium.onnx.json?download=true"
		},
		"Aru - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/aru/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/aru/medium/en_GB-aru-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/aru/medium/en_GB-aru-medium.onnx.json?download=true"
		},
		"Cori - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/medium/en_GB-cori-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/medium/en_GB-cori-medium.onnx.json?download=true"
		},
		"Cori - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/high/en_GB-cori-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/cori/high/en_GB-cori-high.onnx.json?download=true"
		},
		"Jenny (Dioco) - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/jenny_dioco/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/jenny_dioco/medium/en_GB-jenny_dioco-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/jenny_dioco/medium/en_GB-jenny_dioco-medium.onnx.json?download=true"
		},
		"Northern English Male - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/northern_english_male/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/northern_english_male/medium/en_GB-northern_english_male-medium.onnx.json?download=true"
		},
		"Semaine - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/semaine/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/semaine/medium/en_GB-semaine-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/semaine/medium/en_GB-semaine-medium.onnx.json?download=true"
		},
		"Southern English Female - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/southern_english_female/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/southern_english_female/low/en_GB-southern_english_female-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/southern_english_female/low/en_GB-southern_english_female-low.onnx.json?download=true"
		},
		"VCTK - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/vctk/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/vctk/medium/en_GB-vctk-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_GB/vctk/medium/en_GB-vctk-medium.onnx.json?download=true"
		}
	},
	"en_US" => phf::phf_map! {
		"Amy - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/low/en_US-amy-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/low/en_US-amy-low.onnx.json?download=true"
		},
		"Amy - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/medium/en_US-amy-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/amy/medium/en_US-amy-medium.onnx.json?download=true"
		},
		"Arctic - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/arctic/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/arctic/medium/en_US-arctic-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/arctic/medium/en_US-arctic-medium.onnx.json?download=true"
		},
		"Bryce - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/bryce/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/bryce/medium/en_US-bryce-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/bryce/medium/en_US-bryce-medium.onnx.json?download=true"
		},
		"Danny - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/danny/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/danny/low/en_US-danny-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/danny/low/en_US-danny-low.onnx.json?download=true"
		},
		"HFC Female - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_female/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_female/medium/en_US-hfc_female-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_female/medium/en_US-hfc_female-medium.onnx.json?download=true"
		},
		"HFC Male - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_male/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_male/medium/en_US-hfc_male-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/hfc_male/medium/en_US-hfc_male-medium.onnx.json?download=true"
		},
		"Joe - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/joe/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/joe/medium/en_US-joe-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/joe/medium/en_US-joe-medium.onnx.json?download=true"
		},
		"John - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/john/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/john/medium/en_US-john-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/john/medium/en_US-john-medium.onnx.json?download=true"
		},
		"Kathleen - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kathleen/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kathleen/low/en_US-kathleen-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kathleen/low/en_US-kathleen-low.onnx.json?download=true"
		},
		"Kristin - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kristin/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kristin/medium/en_US-kristin-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kristin/medium/en_US-kristin-medium.onnx.json?download=true"
		},
		"Kusal - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kusal/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kusal/medium/en_US-kusal-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/kusal/medium/en_US-kusal-medium.onnx.json?download=true"
		},
		"L2Arctic - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/l2arctic/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/l2arctic/medium/en_US-l2arctic-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/l2arctic/medium/en_US-l2arctic-medium.onnx.json?download=true"
		},
		"Lessac - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/low/en_US-lessac-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/low/en_US-lessac-low.onnx.json?download=true"
		},
		"Lessac - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/medium/en_US-lessac-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/medium/en_US-lessac-medium.onnx.json?download=true"
		},
		"Lessac - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/high/en_US-lessac-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/lessac/high/en_US-lessac-high.onnx.json?download=true"
		},
		"Libritts - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts/high/en_US-libritts-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts/high/en_US-libritts-high.onnx.json?download=true"
		},
		"Libritts R - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts_r/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/libritts_r/medium/en_US-libritts_r-medium.onnx.json?download=true"
		},
		"Ljspeech - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/medium/en_US-ljspeech-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/medium/en_US-ljspeech-medium.onnx.json?download=true"
		},
		"Ljspeech - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/high/en_US-ljspeech-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ljspeech/high/en_US-ljspeech-high.onnx.json?download=true"
		},
		"Norman - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/norman/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/norman/medium/en_US-norman-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/norman/medium/en_US-norman-medium.onnx.json?download=true"
		},
		"Reza Ibrahim - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/reza_ibrahim/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/reza_ibrahim/medium/en_US-reza_ibrahim-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/reza_ibrahim/medium/en_US-reza_ibrahim-medium.onnx.json?download=true"
		},
		"Ryan - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/low/en_US-ryan-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/low/en_US-ryan-low.onnx.json?download=true"
		},
		"Ryan - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/medium/en_US-ryan-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/medium/en_US-ryan-medium.onnx.json?download=true"
		},
		"Ryan - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/high/en_US-ryan-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/ryan/high/en_US-ryan-high.onnx.json?download=true"
		},
		"Sam - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/sam/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/sam/medium/en_US-sam-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/en/en_US/sam/medium/en_US-sam-medium.onnx.json?download=true"
		}
	},
	"es_ES" => phf::phf_map! {
		"CarlFM - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/carlfm/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/carlfm/x_low/es_ES-carlfm-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/carlfm/x_low/es_ES-carlfm-x_low.onnx.json?download=true"
		},
		"DaveFX - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/davefx/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/davefx/medium/es_ES-davefx-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/davefx/medium/es_ES-davefx-medium.onnx.json?download=true"
		},
		"MLS 10246 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_10246/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_10246/low/es_ES-mls_10246-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_10246/low/es_ES-mls_10246-low.onnx.json?download=true"
		},
		"MLS 9972 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_9972/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_9972/low/es_ES-mls_9972-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/mls_9972/low/es_ES-mls_9972-low.onnx.json?download=true"
		},
		"Sharvard - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/sharvard/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/sharvard/medium/es_ES-sharvard-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_ES/sharvard/medium/es_ES-sharvard-medium.onnx.json?download=true"
		}
	},
	"es_MX" => phf::phf_map! {
		"Ald - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/ald/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/ald/medium/es_MX-ald-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/ald/medium/es_MX-ald-medium.onnx.json?download=true"
		},
		"Claude - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/claude/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/claude/high/es_MX-claude-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/es/es_MX/claude/high/es_MX-claude-high.onnx.json?download=true"
		}
	},
	"fa_IR" => phf::phf_map! {
		"Amir - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/amir/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/amir/medium/fa_IR-amir-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/amir/medium/fa_IR-amir-medium.onnx.json?download=true"
		},
		"Ganji - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji/medium/fa_IR-ganji-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji/medium/fa_IR-ganji-medium.onnx.json?download=true"
		},
		"Ganji Adabi - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji_adabi/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji_adabi/medium/fa_IR-ganji_adabi-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/ganji_adabi/medium/fa_IR-ganji_adabi-medium.onnx.json?download=true"
		},
		"Gyro - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/gyro/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/gyro/medium/fa_IR-gyro-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/gyro/medium/fa_IR-gyro-medium.onnx.json?download=true"
		},
		"Reza Ibrahim - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/reza_ibrahim/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/reza_ibrahim/medium/fa_IR-reza_ibrahim-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fa/fa_IR/reza_ibrahim/medium/fa_IR-reza_ibrahim-medium.onnx.json?download=true"
		}
	},
	"fi_FI" => phf::phf_map! {
		"Harri - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/low/fi_FI-harri-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/low/fi_FI-harri-low.onnx.json?download=true"
		},
		"Harri - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/medium/fi_FI-harri-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fi/fi_FI/harri/medium/fi_FI-harri-medium.onnx.json?download=true"
		}
	},
	"fr_FR" => phf::phf_map! {
		"Gilles - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/gilles/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/gilles/low/fr_FR-gilles-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/gilles/low/fr_FR-gilles-low.onnx.json?download=true"
		},
		"MLS - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls/medium/fr_FR-mls-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls/medium/fr_FR-mls-medium.onnx.json?download=true"
		},
		"MLS 1840 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls_1840/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls_1840/low/fr_FR-mls_1840-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/mls_1840/low/fr_FR-mls_1840-low.onnx.json?download=true"
		},
		"Siwis - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/low/fr_FR-siwis-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/low/fr_FR-siwis-low.onnx.json?download=true"
		},
		"Siwis - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/medium/fr_FR-siwis-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/siwis/medium/fr_FR-siwis-medium.onnx.json?download=true"
		},
		"Tom - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/tom/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/tom/medium/fr_FR-tom-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/tom/medium/fr_FR-tom-medium.onnx.json?download=true"
		},
		"UPMC - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/upmc/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/upmc/medium/fr_FR-upmc-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/fr/fr_FR/upmc/medium/fr_FR-upmc-medium.onnx.json?download=true"
		}
	},
	"hi_IN" => phf::phf_map! {
		"Pratham - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/pratham/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/pratham/medium/hi_IN-pratham-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/pratham/medium/hi_IN-pratham-medium.onnx.json?download=true"
		},
		"Priyamvada - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/priyamvada/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/priyamvada/medium/hi_IN-priyamvada-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hi/hi_IN/priyamvada/medium/hi_IN-priyamvada-medium.onnx.json?download=true"
		}
	},
	"hu_HU" => phf::phf_map! {
		"Anna - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/anna/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/anna/medium/hu_HU-anna-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/anna/medium/hu_HU-anna-medium.onnx.json?download=true"
		},
		"Berta - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/berta/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/berta/medium/hu_HU-berta-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/berta/medium/hu_HU-berta-medium.onnx.json?download=true"
		},
		"Imre - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/imre/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/imre/medium/hu_HU-imre-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/hu/hu_HU/imre/medium/hu_HU-imre-medium.onnx.json?download=true"
		}
	},
	"is_IS" => phf::phf_map! {
		"Bui - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/bui/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/bui/medium/is_IS-bui-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/bui/medium/is_IS-bui-medium.onnx.json?download=true"
		},
		"Salka - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/salka/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/salka/medium/is_IS-salka-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/salka/medium/is_IS-salka-medium.onnx.json?download=true"
		},
		"Steinn - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/steinn/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/steinn/medium/is_IS-steinn-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/steinn/medium/is_IS-steinn-medium.onnx.json?download=true"
		},
		"Ugla - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/ugla/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/ugla/medium/is_IS-ugla-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/is/is_IS/ugla/medium/is_IS-ugla-medium.onnx.json?download=true"
		}
	},
	"it_IT" => phf::phf_map! {
		"Paola - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/paola/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/paola/medium/it_IT-paola-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/paola/medium/it_IT-paola-medium.onnx.json?download=true"
		},
		"Riccardo - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/riccardo/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/riccardo/x_low/it_IT-riccardo-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/it/it_IT/riccardo/x_low/it_IT-riccardo-x_low.onnx.json?download=true"
		}
	},
	"ka_GE" => phf::phf_map! {
		"Natia - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ka/ka_GE/natia/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ka/ka_GE/natia/medium/ka_GE-natia-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ka/ka_GE/natia/medium/ka_GE-natia-medium.onnx.json?download=true"
		}
	},
	"kk_KZ" => phf::phf_map! {
		"Iseke - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/iseke/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/iseke/x_low/kk_KZ-iseke-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/iseke/x_low/kk_KZ-iseke-x_low.onnx.json?download=true"
		},
		"Issai - High" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/issai/high/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/issai/high/kk_KZ-issai-high.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/issai/high/kk_KZ-issai-high.onnx.json?download=true"
		},
		"Raya - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/raya/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/raya/x_low/kk_KZ-raya-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/kk/kk_KZ/raya/x_low/kk_KZ-raya-x_low.onnx.json?download=true"
		}
	},
	"lb_LU" => phf::phf_map! {
		"Marylux - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lb/lb_LU/marylux/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lb/lb_LU/marylux/medium/lb_LU-marylux-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lb/lb_LU/marylux/medium/lb_LU-marylux-medium.onnx.json?download=true"
		}
	},
	"lv_LV" => phf::phf_map! {
		"Aivars - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lv/lv_LV/aivars/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lv/lv_LV/aivars/medium/lv_LV-aivars-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/lv/lv_LV/aivars/medium/lv_LV-aivars-medium.onnx.json?download=true"
		}
	},
	"ml_IN" => phf::phf_map! {
		"Arjun - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/arjun/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/arjun/medium/ml_IN-arjun-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/arjun/medium/ml_IN-arjun-medium.onnx.json?download=true"
		},
		"Meera - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/meera/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/meera/medium/ml_IN-meera-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ml/ml_IN/meera/medium/ml_IN-meera-medium.onnx.json?download=true"
		}
	},
	"ne_NP" => phf::phf_map! {
		"Chitwan - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/chitwan/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/chitwan/medium/ne_NP-chitwan-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/chitwan/medium/ne_NP-chitwan-medium.onnx.json?download=true"
		},
		"Google - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/x_low/ne_NP-google-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/x_low/ne_NP-google-x_low.onnx.json?download=true"
		},
		"Google - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/medium/ne_NP-google-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ne/ne_NP/google/medium/ne_NP-google-medium.onnx.json?download=true"
		}
	},
	"nl_NL" => phf::phf_map! {
		"MLS - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls/medium/nl_NL-mls-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls/medium/nl_NL-mls-medium.onnx.json?download=true"
		},
		"MLS 5809 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_5809/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_5809/low/nl_NL-mls_5809-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_5809/low/nl_NL-mls_5809-low.onnx.json?download=true"
		},
		"MLS 7432 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_7432/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_7432/low/nl_NL-mls_7432-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/mls_7432/low/nl_NL-mls_7432-low.onnx.json?download=true"
		},
		"Pim - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/pim/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/pim/medium/nl_NL-pim-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/pim/medium/nl_NL-pim-medium.onnx.json?download=true"
		},
		"Ronnie - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/ronnie/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/ronnie/medium/nl_NL-ronnie-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/nl/nl_NL/ronnie/medium/nl_NL-ronnie-medium.onnx.json?download=true"
		}
	},
	"no_NO" => phf::phf_map! {
		"Talesyntese - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/no/no_NO/talesyntese/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/no/no_NO/talesyntese/medium/no_NO-talesyntese-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/no/no_NO/talesyntese/medium/no_NO-talesyntese-medium.onnx.json?download=true"
		}
	},
	"pl_PL" => phf::phf_map! {
		"Darkman - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/darkman/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/darkman/medium/pl_PL-darkman-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/darkman/medium/pl_PL-darkman-medium.onnx.json?download=true"
		},
		"Gosia - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/gosia/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/gosia/medium/pl_PL-gosia-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/gosia/medium/pl_PL-gosia-medium.onnx.json?download=true"
		},
		"Mc Speech - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mc_speech/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mc_speech/medium/pl_PL-mc_speech-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mc_speech/medium/pl_PL-mc_speech-medium.onnx.json?download=true"
		},
		"MLS 6892 - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mls_6892/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mls_6892/low/pl_PL-mls_6892-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pl/pl_PL/mls_6892/low/pl_PL-mls_6892-low.onnx.json?download=true"
		}
	},
	"pt_BR" => phf::phf_map! {
		"Cadu - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/cadu/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/cadu/medium/pt_BR-cadu-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/cadu/medium/pt_BR-cadu-medium.onnx.json?download=true"
		},
		"Edresson - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/edresson/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/edresson/low/pt_BR-edresson-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/edresson/low/pt_BR-edresson-low.onnx.json?download=true"
		},
		"Faber - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/faber/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/faber/medium/pt_BR-faber-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/faber/medium/pt_BR-faber-medium.onnx.json?download=true"
		},
		"Jeff - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/jeff/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/jeff/medium/pt_BR-jeff-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_BR/jeff/medium/pt_BR-jeff-medium.onnx.json?download=true"
		}
	},
	"pt_PT" => phf::phf_map! {
		"Tugão - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_PT/tugão/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_PT/tugão/medium/pt_PT-tugão-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/pt/pt_PT/tugão/medium/pt_PT-tugão-medium.onnx.json?download=true"
		}
	},
	"ro_RO" => phf::phf_map! {
		"Mihai - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ro/ro_RO/mihai/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ro/ro_RO/mihai/medium/ro_RO-mihai-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ro/ro_RO/mihai/medium/ro_RO-mihai-medium.onnx.json?download=true"
		}
	},
	"ru_RU" => phf::phf_map! {
		"Denis - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/denis/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/denis/medium/ru_RU-denis-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/denis/medium/ru_RU-denis-medium.onnx.json?download=true"
		},
		"Dmitri - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/dmitri/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/dmitri/medium/ru_RU-dmitri-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/dmitri/medium/ru_RU-dmitri-medium.onnx.json?download=true"
		},
		"Irina - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/irina/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/irina/medium/ru_RU-irina-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/irina/medium/ru_RU-irina-medium.onnx.json?download=true"
		},
		"Ruslan - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/ruslan/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/ruslan/medium/ru_RU-ruslan-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/ru/ru_RU/ruslan/medium/ru_RU-ruslan-medium.onnx.json?download=true"
		}
	},
	"sk_SK" => phf::phf_map! {
		"Lili - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sk/sk_SK/lili/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sk/sk_SK/lili/medium/sk_SK-lili-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sk/sk_SK/lili/medium/sk_SK-lili-medium.onnx.json?download=true"
		}
	},
	"sl_SI" => phf::phf_map! {
		"Artur - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sl/sl_SI/artur/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sl/sl_SI/artur/medium/sl_SI-artur-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sl/sl_SI/artur/medium/sl_SI-artur-medium.onnx.json?download=true"
		}
	},
	"sr_RS" => phf::phf_map! {
		"Serbski Institut - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sr/sr_RS/serbski_institut/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sr/sr_RS/serbski_institut/medium/sr_RS-serbski_institut-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sr/sr_RS/serbski_institut/medium/sr_RS-serbski_institut-medium.onnx.json?download=true"
		}
	},
	"sv_SE" => phf::phf_map! {
		"Lisa - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/lisa/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/lisa/medium/sv_SE-lisa-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/lisa/medium/sv_SE-lisa-medium.onnx.json?download=true"
		},
		"Nst - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/nst/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/nst/medium/sv_SE-nst-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sv/sv_SE/nst/medium/sv_SE-nst-medium.onnx.json?download=true"
		}
	},
	"sw_CD" => phf::phf_map! {
		"Lanfrica - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sw/sw_CD/lanfrica/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sw/sw_CD/lanfrica/medium/sw_CD-lanfrica-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/sw/sw_CD/lanfrica/medium/sw_CD-lanfrica-medium.onnx.json?download=true"
		}
	},
	"tr_TR" => phf::phf_map! {
		"Dfki - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/dfki/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/dfki/medium/tr_TR-dfki-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/dfki/medium/tr_TR-dfki-medium.onnx.json?download=true"
		},
		"Fahrettin - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fahrettin/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fahrettin/medium/tr_TR-fahrettin-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fahrettin/medium/tr_TR-fahrettin-medium.onnx.json?download=true"
		},
		"Fettah - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fettah/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fettah/medium/tr_TR-fettah-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/tr/tr_TR/fettah/medium/tr_TR-fettah-medium.onnx.json?download=true"
		}
	},
	"uk_UA" => phf::phf_map! {
		"Lada - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/lada/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/lada/x_low/uk_UA-lada-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/lada/x_low/uk_UA-lada-x_low.onnx.json?download=true"
		},
		"Ukrainian TTS - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/ukrainian_tts/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/ukrainian_tts/medium/uk_UA-ukrainian_tts-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/uk/uk_UA/ukrainian_tts/medium/uk_UA-ukrainian_tts-medium.onnx.json?download=true"
		}
	},
	"vi_VN" => phf::phf_map! {
		"25 Hours Single - Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/25hours_single/low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/25hours_single/low/vi_VN-25hours_single-low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/25hours_single/low/vi_VN-25hours_single-low.onnx.json?download=true"
		},
		"Vais1000 - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vais1000/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vais1000/medium/vi_VN-vais1000-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vais1000/medium/vi_VN-vais1000-medium.onnx.json?download=true"
		},
		"Vivos - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vivos/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vivos/x_low/vi_VN-vivos-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/vi/vi_VN/vivos/x_low/vi_VN-vivos-x_low.onnx.json?download=true"
		}
	},
	"zh_CN" => phf::phf_map! {
		"Huayan - Very Low" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/x_low/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/x_low/zh_CN-huayan-x_low.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/x_low/zh_CN-huayan-x_low.onnx.json?download=true"
		},
		"Huayan - Medium" => PiperVoiceUrls {
			example: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/medium/samples/speaker_:speaker_id:.mp3?download=true",
			onnx: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/medium/zh_CN-huayan-medium.onnx?download=true",
			json: "https://huggingface.co/rhasspy/piper-voices/resolve/v1.0.0/zh/zh_CN/huayan/medium/zh_CN-huayan-medium.onnx.json?download=true"
		}
	},
};
