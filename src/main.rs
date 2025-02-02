use color_eyre::eyre::Result;
use tedbot::{auth, chat, cli::Config, install_tracing, print_channel_info};

#[tokio::main]
async fn main() -> Result<()> {
	install_tracing();
	color_eyre::install()?;

	let conf = Config::get()?;

	let token = auth::twitch_auth().await?;

	print_channel_info(conf.username.as_ref(), &token).await?;

	todo!("Use Token {:#?}", &token);

	chat::chat().await?;

	Ok(())
}
