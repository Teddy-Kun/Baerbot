use eyre::{Result, eyre};
use log::warn;
use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use std::{
	fmt::Debug,
	path::Path,
	sync::{Arc, Mutex},
	time::Duration,
};
use tokio::{task::JoinHandle, time::Instant};

use crate::shared::cfg::Config;

type CallbackHandle = Arc<Mutex<Option<JoinHandle<()>>>>;
#[derive(Clone)]
pub struct Tts {
	synth: Arc<PiperSpeechSynthesizer>,
	timeout: Option<u16>,
	queue: Vec<Box<str>>,
	last_played: Option<Instant>,
	callback_handle: CallbackHandle,
}

impl Debug for Tts {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Tts timeout: {:?}, last_played: {:?}, queue: {:?}",
			self.timeout, self.last_played, self.queue
		)
	}
}

impl Tts {
	pub fn new(model: &str, timeout: Option<u16>) -> Result<Self> {
		let model = piper_rs::from_config_path(Path::new(model))?;
		let synth = PiperSpeechSynthesizer::new(model)?;
		Ok(Self {
			synth: Arc::new(synth),
			timeout,
			queue: Vec::new(),
			last_played: None,
			callback_handle: Arc::new(Mutex::new(None)),
		})
	}

	fn setup_callback(&mut self) -> Result<()> {
		let handle_clone = self.callback_handle.clone();
		let handle = handle_clone.lock().unwrap();
		if handle.is_some() {
			return Err(eyre!("Callback already running"));
		}

		if self.timeout.is_none() {
			return Err(eyre!("No timeout is given"));
		}

		if self.last_played.is_none() {
			return Err(eyre!("Nothing was ever played"));
		}

		let deadline =
			self.last_played.unwrap() + Duration::from_secs(self.timeout.unwrap() as u64);

		warn!("WIP: callback to play sound in {:?}s", deadline);

		Ok(())
	}

	pub fn add_to_queue(&mut self, text: &str) -> Result<()> {
		let boxed_text: Box<str> = text.into();
		if self.queue.len() == 0 {
			if self.last_played.is_some() && self.timeout.is_some() {
				let last_played = self.last_played.unwrap();
				let timeout = self.timeout.unwrap();

				if last_played.elapsed().as_secs() < timeout as u64 {
					self.queue.push(boxed_text);
					self.setup_callback()?;
					return Ok(());
				} else {
					self.play_tts(boxed_text.as_ref())?;
					return Ok(());
				}
			} else {
				self.last_played = Some(Instant::now());
				self.play_tts(boxed_text.as_ref())?;
			}

			return Ok(());
		}

		self.queue.push(boxed_text);

		Ok(())
	}

	fn play_tts(&mut self, msg: &str) -> Result<()> {
		let mut samples: Vec<f32> = Vec::new();
		let audio = self.synth.synthesize_parallel(msg.into(), None).unwrap();
		for result in audio {
			if let Ok(res) = result {
				samples.append(&mut res.into_vec());
			}
		}

		let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
		let sink = rodio::Sink::try_new(&handle).unwrap();

		self.last_played = Some(Instant::now());

		let buf = SamplesBuffer::new(1, 22050, samples);
		sink.append(buf);

		sink.sleep_until_end();

		Ok(())
	}
}

pub fn setup_tts(cfg: &Config) -> Result<Tts> {
	match cfg.tts_model.clone() {
		None => Err(eyre!("Missing Model for TTS")),
		Some(model) => match model.to_str() {
			None => Err(eyre!("Couldn't convert model path to str")),
			Some(m) => {
				let mut tts_instance = Tts::new(m, None)?;
				tts_instance.add_to_queue("TTS initialized")?;
				Ok(tts_instance)
			}
		},
	}
}
