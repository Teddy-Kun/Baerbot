use eyre::{eyre, Result};
use serde::Deserialize;
use std::net::TcpListener;
use std::{io::Read, net::TcpStream};
use tracing::{error, trace};
use url::Url;

#[derive(Debug, Default, Deserialize)]
pub struct TwitchAuthParams {
	pub code: Option<String>,
	pub scope: Option<String>,
	pub state: Option<String>,
	pub error: Option<String>,
	pub error_description: Option<String>,
}

pub fn parse_auth_code(stream: &mut TcpStream) -> Result<TwitchAuthParams> {
	let mut buffer = String::new();
	stream.read_to_string(&mut buffer)?;

	trace!("buffer:\n{}", buffer);

	let mut headers = [httparse::EMPTY_HEADER; 16];
	let mut req = httparse::Request::new(&mut headers);

	if req.parse(&buffer.as_bytes()).is_ok() {
		if let Some(path) = req.path {
			let url_str = format!("http://localhost{}", path); // Dummy host needed for parsing
			if let Ok(url) = Url::parse(&url_str) {
				let query_params: TwitchAuthParams =
					serde_urlencoded::from_str(url.query().unwrap_or(""))?;
				return Ok(query_params);
			}
		}
	}

	Err(eyre!("Failed to parse response"))
}

pub async fn get_auth_code() -> Result<TwitchAuthParams> {
	let listener = TcpListener::bind("127.0.0.1:35594")?;

	// see https://dev.twitch.tv/docs/authentication/scopes/
	let scopes = ["chat:edit", "chat:read"];

	let params = [
		("client_id", "15xr4zw5ue7jxpbvt0jwwrwywqch9a"),
		("redirect_uri", "http://localhost:35594/"),
		("response_type", "code"),
		("scope", &scopes.join(" ")),
	];

	let q = serde_urlencoded::to_string(params)?;
	let url = format!("https://id.twitch.tv/oauth2/authorize?{}", q);

	open::that(url.as_str())?;

	for stream in listener.incoming() {
		match stream {
			Ok(mut stream) => {
				let auth = parse_auth_code(&mut stream)?;

				// TODO: respond so that the browser can close gracefully

				return Ok(auth);
			}
			Err(e) => {
				error!("Error: {}", e);
			}
		}
	}

	Err(eyre!("Failed to get auth code"))
}
