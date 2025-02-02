use color_eyre::eyre::Result;
use tedbot::{init, install_tracing};

#[tokio::main]
async fn main() -> Result<()> {
	install_tracing();
	color_eyre::install()?;

	init().await?;

	Ok(())
}
