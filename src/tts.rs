use eyre::Result;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use std::path::Path;

pub fn setup_tts() -> Result<()> {
	const DBG_CFG: &str = "piper/en_US-amy-medium.onnx.json";
	// H.P. Lovecraft The Unnameable
	const DBG_TEXT: &str = "No—it wasn’t that way at all. It was everywhere—a gelatin—a slime—yet it had shapes, a thousand shapes of horror beyond all memory. There were eyes—and a blemish. It was the pit—the maelstrom—the ultimate abomination. Carter, it was the unnamable!";
	let model = piper_rs::from_config_path(Path::new(DBG_CFG))?;
	let synth = PiperSpeechSynthesizer::new(model)?;

	let mut samples: Vec<f32> = Vec::new();
	let audio = synth.synthesize_parallel(DBG_TEXT.into(), None).unwrap();
	for result in audio {
		samples.append(&mut result.unwrap().into_vec());
	}

	let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
	let sink = rodio::Sink::try_new(&handle).unwrap();

	let buf = SamplesBuffer::new(1, 22050, samples);
	sink.append(buf);

	sink.sleep_until_end();
	Ok(())
}
