use color_eyre::eyre::Result;
use tedbot::{auth, chat, init, install_tracing};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<()> {
	install_tracing();
	color_eyre::install()?;

	let auth_code = auth::get_auth_code().await?;
	debug!("Auth code: {:#?}", auth_code);

	todo!("Get access token from auth code");

	chat::chat().await?;

	init().await?;

	Ok(())
}
