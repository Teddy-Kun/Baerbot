use eyre::Result;
use log::info;
use std::process;
use tauri::{
	App,
	menu::{Menu, MenuEvent, MenuItem},
	tray::TrayIconBuilder,
};

pub fn init_tray(app: &mut App) -> Result<()> {
	let show = MenuItem::with_id(app, "show", "Show UI", true, None::<&str>)?;
	let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
	let menu = Menu::with_items(app, &[&show, &quit_i])?;

	TrayIconBuilder::new()
		.menu(&menu)
		.show_menu_on_left_click(true)
		.on_menu_event(|_app, event| tray_event_handler(event))
		.icon(app.default_window_icon().unwrap().clone())
		.build(app)?;
	Ok(())
}

fn tray_event_handler(event: MenuEvent) {
	match event.id.as_ref() {
		"show" => info!("TODO: show/hide"),
		"quit" => process::exit(0),
		_ => info!("unhandled event {:?}", event),
	}
}
