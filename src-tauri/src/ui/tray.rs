use eyre::Result;
use tauri::{
	App,
	menu::{Menu, MenuEvent, MenuItem},
	tray::TrayIconBuilder,
};
use tracing::info;

pub fn init_tray(app: &mut App) -> Result<()> {
	let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
	let menu = Menu::with_items(app, &[&quit_i])?;

	TrayIconBuilder::new()
		.menu(&menu)
		.show_menu_on_left_click(true)
		.on_menu_event(|_app, event| tray_event_handler(event))
		.icon(app.default_window_icon().unwrap().clone())
		.build(app)?;
	Ok(())
}

fn tray_event_handler(event: MenuEvent) {
	info!("event ({:?})", event)
}
