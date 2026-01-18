use colors_transform::{Color as Col, Rgb};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct ColorSchemeAccent {
	hue: f32,
	saturation: f32,
	luminance: f32,
	hex_code: String,
}

impl ColorSchemeAccent {
	pub fn new(red: u8, green: u8, blue: u8) -> ColorSchemeAccent {
		let color = Rgb::from(red as f32, green as f32, blue as f32);
		let hue = color.get_hue();
		let saturation = color.get_saturation();
		let luminance = color.get_lightness();

		ColorSchemeAccent {
			hue,
			saturation,
			luminance,
			hex_code: format!("#{:02x}{:02x}{:02x}", red, green, blue),
		}
	}
}

#[cfg(target_os = "linux")]
impl From<ashpd::desktop::Color> for ColorSchemeAccent {
	fn from(value: ashpd::desktop::Color) -> Self {
		ColorSchemeAccent::new(
			(255.0 * value.red()) as u8,
			(255.0 * value.green()) as u8,
			(255.0 * value.blue()) as u8,
		)
	}
}

pub async fn get_color_scheme() -> Option<ColorSchemeAccent> {
	#[cfg(target_os = "linux")]
	{
		let setts = ashpd::desktop::settings::Settings::new().await.ok()?;
		let accent: ColorSchemeAccent = setts.accent_color().await.ok()?.into();

		Some(accent)
	}

	#[cfg(target_os = "windows")]
	{
		get_accent_color_windows().ok()
	}

	#[cfg(not(any(target_os = "windows", target_os = "linux")))]
	{
		None
	}
}

#[cfg(target_os = "windows")]
fn get_accent_color_windows() -> Result<ColorSchemeAccent, crate::error::Error> {
	let mut colorization: u32 = 0;
	let mut opaqueblend = windows::core::BOOL(0);
	unsafe {
		windows::Win32::Graphics::Dwm::DwmGetColorizationColor(
			&mut colorization,
			&mut opaqueblend,
		)?;
	}

	let argb = hex::decode(format!("{:X}", colorization))?;
	println!("argb {:?}", argb);

	Ok(ColorSchemeAccent::new(argb[1], argb[2], argb[3]))
}
