use tauri_specta::{Builder, collect_commands};
use tedbot_lib::{
	error::ErrorMsg,
	os_color::{ColorSchemeAccent, get_color_scheme},
	twitch::{
		self, TWITCH_CLIENT,
		actions::Action,
		auth::{forget_token, load_token},
	},
};

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
async fn login() -> Result<String, ErrorMsg> {
	let reader = TWITCH_CLIENT.read().await;

	if reader.get_username().is_some() {
		return Err(ErrorMsg::AlreadyLoggedIn);
	}

	let tkn = reader.login().await?;

	drop(reader); // drop read lock, because we need to write

	let name = tkn.login.to_string();

	let mut client_writer = TWITCH_CLIENT.write().await;
	client_writer.set_token(tkn).await;

	Ok(name)
}

#[tauri::command]
#[specta::specta]
async fn is_logged_in() -> Option<String> {
	let reader = TWITCH_CLIENT.read().await;
	let maybe = reader.get_username();
	match maybe {
		None => match load_token(&reader).await {
			Err(_) => None,
			Ok(tkn) => {
				drop(reader); // drop read lock, because we need to write

				let name = tkn.login.to_string();
				TWITCH_CLIENT.write().await.set_token(tkn).await;
				Some(name)
			}
		},
		Some(n) => Some(n),
	}
}

#[tauri::command]
#[specta::specta]
async fn logout() {
	if let Err(e) = forget_token().await {
		tracing::error!("Error logging out: {e}");
	}
}

#[tauri::command]
#[specta::specta]
async fn get_accent_color() -> Option<ColorSchemeAccent> {
	get_color_scheme().await
}

#[tauri::command]
#[specta::specta]
async fn get_all_actions() -> Vec<Action> {
	twitch::actions::get_all_actions().await
}

#[tauri::command]
#[specta::specta]
async fn add_action(action: Action) {
	tracing::info!("Saving action: {action:?}");
	twitch::actions::add_action(action).await
}

#[tauri::command]
#[specta::specta]
async fn remove_action(trigger: String) {
	tracing::info!("Removing action: {trigger}");
	twitch::actions::drop_action(trigger.as_str()).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::new().commands(collect_commands![
		greet,
		login,
		is_logged_in,
		logout,
		get_accent_color,
		get_all_actions,
		add_action,
		remove_action,
	]);

	#[cfg(debug_assertions)] // <- Only export on non-release builds
	{
		use specta_typescript::Typescript;

		builder
			.export(Typescript::default(), "../src/lib/bindings.ts")
			.expect("Failed to export typescript bindings");
	}

	tauri::Builder::default()
		// .plugin(tauri_plugin_opener::init())
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			// This is required if you want to use events
			builder.mount_events(app);
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
