use eyre::Result;

mod service;
mod shared;
mod ui;

pub async fn start_service() -> Result<()> {
	service::start_service().await
}

pub fn start_ui() {
	ui::run();
}
