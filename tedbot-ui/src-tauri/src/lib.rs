use tedbot_shared::{
	data,
	tauri_specta::{Builder, collect_commands},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
fn test() -> data::SimpleResponse {
	data::SimpleResponse {
		trigger: "test".to_string(),
		response: "test".to_string(),
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let ts_builder = Builder::<tauri::Wry>::new().commands(collect_commands![greet, test]);
	#[cfg(debug_assertions)] // <- Only export on non-release builds
	{
		use specta_typescript::Typescript;
		ts_builder
			.export(Typescript::default(), "../src/bindings.ts")
			.expect("Failed to export typescript bindings");
	}
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(ts_builder.invoke_handler())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
