use keyring::KeyringEntry;
use std::{
	fs::{self, File},
	io::Write,
};
use tokio::time::sleep;
use twitch_oauth2::{AccessToken, ClientId, DeviceUserTokenBuilder, UserToken};

use crate::{
	config::{ARGS, Config},
	error::{Error, ErrorMsg},
	twitch::TwitchClient,
};

impl TwitchClient {
	pub async fn login(&self) -> Result<UserToken, ErrorMsg> {
		let token = match load_token(self).await {
			Ok(tkn) => tkn,
			Err(_) => twitch_auth(self).await?, // why we failed, doesn't really matter, just log in with the browser
		};

		Ok(token)
	}
}

pub async fn twitch_auth(client: &TwitchClient) -> Result<UserToken, Error> {
	let tkn = internal_twitch_auth(client)
		.await
		.map_err(|e| e.try_set_msg(ErrorMsg::TwitchAuth))?;
	Ok(tkn)
}

async fn internal_twitch_auth(client: &TwitchClient) -> Result<UserToken, Error> {
	let client_id = ClientId::new(env!("API_KEY").to_string());

	// unwrap is safe here, default Config has scopes
	let mut builder = DeviceUserTokenBuilder::new(client_id, Config::default().scopes.unwrap());
	let code = builder.start(&client.client).await?;

	tracing::debug!("code: {:#?}", code);

	let url = code.verification_uri.clone();
	open::that(url)?;

	let token = builder.wait_for_code(&client.client, sleep).await?;

	if let Err(e) = save_token(&token).await {
		tracing::warn!("Failed to save token: {:#?}", e);
	};

	Ok(token)
}

async fn save_token(token: &UserToken) -> Result<(), Error> {
	internal_save_token(token)
		.await
		.map_err(|e| e.try_set_msg(ErrorMsg::TokenSave))?;
	Ok(())
}

async fn internal_save_token(token: &UserToken) -> Result<(), Error> {
	let conf = &ARGS;

	if conf.temp_token {
		return Err(Error::new(ErrorMsg::FeatureDisabled));
	}

	if let Some(file) = conf.token_file.clone() {
		let mut f = File::create(file.as_ref())?;
		f.write_all(token.access_token.as_str().as_bytes())?;
		return Ok(());
	}

	let entry = KeyringEntry::try_new("access_token")?;
	entry.set_secret(token.access_token.as_str()).await?;

	Ok(())
}

pub async fn forget_token() -> Result<(), Error> {
	let entry = KeyringEntry::try_new("access_token")?;
	KeyringEntry::delete_secret(&entry).await?;

	Ok(())
}

pub async fn load_token(client: &TwitchClient) -> Result<UserToken, Error> {
	let tkn = internal_load_token(client)
		.await
		.map_err(|e| e.try_set_msg(ErrorMsg::TokenLoad))?;
	Ok(tkn)
}

async fn internal_load_token(client: &TwitchClient) -> Result<UserToken, Error> {
	let conf = &ARGS;

	if conf.temp_token {
		return Err(Error::new(ErrorMsg::FeatureDisabled));
	}

	if let Some(file) = conf.token_file.clone() {
		let token_str = fs::read_to_string(file.as_ref())?;

		return Ok(UserToken::from_token(&client.client, AccessToken::new(token_str)).await?);
	}

	let entry = KeyringEntry::try_new("access_token")?;
	let access_token = entry.get_secret().await?;
	let access_token = AccessToken::new(access_token);

	let token = UserToken::from_token(&client.client, access_token).await?;

	Ok(token)
}
