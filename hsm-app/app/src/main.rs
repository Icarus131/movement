use app::config::settings::Config;
use app::generator::random_generator::RandomGenerator;
use app::hsm::hashicorp::HashicorpHsm;
use app::processor::message_processor::MessageProcessor;
use app::types::types::Message;
use env_logger;
use log::{error, info};
use std::error::Error;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	env_logger::init();

	let config = Config::new();
	info!("config: {:?}", config);

	let hsm = HashicorpHsm::initialize(&config.addr, &config.auth_token, &config.key_name)?;
	info!("hsm init");

	// create mpsc channel for message passing
	let (tx, rx) = mpsc::channel::<Message>(100);

	let mut generator = RandomGenerator::new(tx, 2);
	tokio::spawn(async move {
		if let Err(e) = generator.start().await {
			error!("generator error!: {}", e);
		}
	});
	info!("generator init");

	let processor = MessageProcessor::new(hsm, rx, 4);
	processor.start().await;

	Ok(())
}

