use keyring::KeyringEntry;
use std::{
	fs::{self, File},
	io::Write,
};
use tokio::time::sleep;
use twitch_api::HelixClient;
use twitch_oauth2::{AccessToken, ClientId, DeviceUserTokenBuilder, Scope, UserToken};

use crate::{
	config::ARGS,
	error::{Error, ErrorMsg},
	twitch::TwitchClient,
};

pub async fn twitch_auth() -> Result<UserToken, Error> {
	let tkn = internal_twitch_auth()
		.await
		.map_err(|e| e.set_msg(ErrorMsg::TwitchAuth))?;
	Ok(tkn)
}

async fn internal_twitch_auth() -> Result<UserToken, Error> {
	let client_id = ClientId::new("15xr4zw5ue7jxpbvt0jwwrwywqch9a".to_string());
	let scopes = vec![Scope::ChatEdit, Scope::ChatRead];

	let client: HelixClient<reqwest::Client> = HelixClient::default();

	let mut builder = DeviceUserTokenBuilder::new(client_id, scopes);
	let code = builder.start(&client).await?;

	tracing::debug!("code: {:#?}", code);

	let url = code.verification_uri.clone();
	open::that(url)?;

	let token = builder.wait_for_code(&client, sleep).await?;

	if let Err(e) = save_token(&token).await {
		tracing::warn!("Failed to save token: {:#?}", e);

		// Panic until I figure out why its broken
		panic!("Failed to save token: {:#?}", e);
	};

	Ok(token)
}

async fn save_token(token: &UserToken) -> Result<(), Error> {
	internal_save_token(token)
		.await
		.map_err(|e| e.set_msg(ErrorMsg::TokenSave))?;
	Ok(())
}

async fn internal_save_token(token: &UserToken) -> Result<(), Error> {
	let conf = &ARGS;
	if let Some(file) = conf.token_file.clone() {
		let mut f = File::create(file.as_ref())?;
		f.write_all(token.access_token.as_str().as_bytes())?;
		return Ok(());
	}

	// TODO: figure out why this dies on Linux, KDE issue?

	let entry = KeyringEntry::try_new("access_token")?;
	entry.set_secret(token.access_token.as_str()).await?;

	Ok(())
}

pub async fn load_token(client: &TwitchClient) -> Result<UserToken, Error> {
	let tkn = internal_load_token(client)
		.await
		.map_err(|e| e.set_msg(ErrorMsg::TokenLoad))?;
	Ok(tkn)
}

async fn internal_load_token(client: &TwitchClient) -> Result<UserToken, Error> {
	let conf = &ARGS;
	if let Some(file) = conf.token_file.clone() {
		let token_str = fs::read_to_string(file.as_ref())?;

		return Ok(UserToken::from_token(client, AccessToken::new(token_str)).await?);
	}

	// TODO: figure out why this dies on Linux, KDE issue?

	let entry = KeyringEntry::try_new("access_token")?;
	let access_token = entry.get_secret().await?;
	let access_token = AccessToken::new(access_token);

	let token = UserToken::from_token(client, access_token).await?;

	Ok(token)
}
