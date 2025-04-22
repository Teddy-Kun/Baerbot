use eyre::Result;
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, menu::Menu};

pub fn init_tray() -> Result<TrayIcon> {
	gtk::init()?;

	let tray_menu = Menu::new();

	let icon = Icon::from_rgba(vec![0, 0, 0, 0], 1, 1)?;

	let tray_icon = TrayIconBuilder::new()
		.with_menu(Box::new(tray_menu))
		.with_icon(icon)
		.with_tooltip("Tedbot")
		.build()?;

	Ok(tray_icon)
}
