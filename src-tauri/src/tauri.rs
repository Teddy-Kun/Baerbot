use tauri_specta::{Builder, collect_commands};
use tedbot_lib::{
	error::ErrorMsg,
	twitch::{TWITCH_CLIENT, TwitchClient},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
async fn login() -> Result<(), ErrorMsg> {
	let tkn = TwitchClient::login().await?;
	let b = Box::new(tkn);

	let cl = TWITCH_CLIENT.clone();

	cl.write().await.set_token(b);

	Ok(())
}

#[tauri::command]
#[specta::specta]
async fn is_logged_in() -> bool {
	TWITCH_CLIENT.read().await.is_logged_in()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let builder = Builder::new().commands(collect_commands![greet, login, is_logged_in]);

	#[cfg(debug_assertions)] // <- Only export on non-release builds
	{
		use specta_typescript::Typescript;

		builder
			.export(Typescript::default(), "../src/lib/bindings.ts")
			.expect("Failed to export typescript bindings");
	}

	tauri::Builder::default()
		// .plugin(tauri_plugin_opener::init())
		// and finally tell Tauri how to invoke them
		.invoke_handler(builder.invoke_handler())
		.setup(move |app| {
			// This is also required if you want to use events
			builder.mount_events(app);

			Ok(())
		})
		// on an actual app, remove the string argument
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
