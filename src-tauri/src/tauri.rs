use baerbot_lib::{
	config::CONFIG,
	error::ErrorMsg,
	obs,
	os_color::{ColorSchemeAccent, get_color_scheme},
	twitch::{
		self, TWITCH_CLIENT,
		actions::{Action, toggle_disable_action as toggle_action},
		auth::{forget_token, load_token},
		chat::get_random_chatter,
	},
	utils::{CFG_DIR_PATH, NAME_CAPITALIZED},
};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::{Builder, collect_commands};
use twitch_oauth2::UserToken;

use crate::logs;

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
async fn login() -> Result<String, ErrorMsg> {
	let tkn: UserToken;
	{
		let reader = TWITCH_CLIENT.read().await;

		if reader.get_username().is_some() {
			return Err(ErrorMsg::AlreadyLoggedIn);
		}

		tkn = reader.login().await?;
		// drop read lock, because we need to write
	}

	let name = tkn.login.to_string();

	TWITCH_CLIENT.write().await.set_token(tkn).await;

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
	TWITCH_CLIENT.write().await.forget_token();
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
fn redeems_enabled() -> bool {
	CONFIG.read().enable_redeems
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
async fn remove_action(trigger: Box<str>) {
	tracing::info!("Removing action: {trigger}");
	twitch::actions::drop_action(trigger.as_ref()).await;
}

#[tauri::command]
#[specta::specta]
async fn get_rand_chatter() -> Option<Box<str>> {
	get_random_chatter().await
}

#[tauri::command]
#[specta::specta]
fn open_log_dir() -> Result<(), String> {
	let p = CFG_DIR_PATH.join("logs");
	if let Err(e) = open::that(p) {
		return Err(e.to_string());
	}

	Ok(())
}

#[tauri::command]
#[specta::specta]
fn get_current_logs() -> Vec<Box<str>> {
	let logs = match logs::get_latest_log() {
		Err(e) => {
			tracing::error!("Couldn't get latest log file {e}");
			return Vec::new();
		}
		Ok(v) => match v {
			None => return Vec::new(),
			Some(l) => l,
		},
	};
	logs.split('\n')
		.map_while(|s| {
			if s.is_empty() {
				return None;
			}
			Some(Box::from(s))
		})
		.collect()
}

#[derive(Debug, Deserialize, Serialize, Type)]
struct FrontendRedeem {
	id: String,
	color: String,
	name: String,
	cost: usize,
}

#[tauri::command]
#[specta::specta]
async fn get_redeems() -> Result<Vec<FrontendRedeem>, ErrorMsg> {
	let res = TWITCH_CLIENT.read().await.update_redeems().await;
	match res {
		Err(mut e) => {
			e = e.overwrite_msg(ErrorMsg::RedeemRequest);
			tracing::error!("Error getting twitch redeems: {e}");
			Err(ErrorMsg::RedeemRequest)
		}
		Ok(redeems) => Ok(redeems
			.into_iter()
			.map(|red| FrontendRedeem {
				color: red.background_color,
				cost: red.cost,
				id: red.id.to_string(),
				name: red.title,
			})
			.collect()),
	}
}

#[tauri::command]
#[specta::specta]
async fn toggle_disable_action(key: Box<str>) -> Option<bool> {
	toggle_action(key.as_ref())
}

#[tauri::command]
#[specta::specta]
async fn connect_obs() -> Result<(), ErrorMsg> {
	match obs::websocket::init_websocket().await {
		Ok(()) => Ok(()),
		Err(err) => {
			let err = err.try_set_msg(ErrorMsg::ObsWS);
			tracing::error!("Couldn't connect to OBS: {err}");
			Err(err.msg)
		}
	}
}

#[tauri::command]
#[specta::specta]
async fn init_obs_overlay() -> Result<(), ErrorMsg> {
	match obs::overlay::init_overlay().await {
		Ok(()) => Ok(()),
		Err(err) => {
			let err = err.try_set_msg(ErrorMsg::ObsOverlay);
			tracing::error!("Couldn't host overlay: {err}");
			Err(err.msg)
		}
	}
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
		get_rand_chatter,
		open_log_dir,
		get_current_logs,
		get_redeems,
		toggle_disable_action,
		redeems_enabled,
		connect_obs,
		init_obs_overlay
	]);

	#[cfg(debug_assertions)] // <- Only export on non-release builds
	{
		use specta_typescript::{BigIntExportBehavior, Typescript, formatter::eslint};

		let ts_settings = Typescript::default()
			.bigint(BigIntExportBehavior::BigInt)
			.formatter(eslint);

		builder
			.export(ts_settings, "../src/lib/bindings.ts")
			.expect("Failed to export typescript bindings");
	}

	tracing::info!("{} started", NAME_CAPITALIZED);

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
