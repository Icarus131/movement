use crate::types::types::{Bytes, Message};
use rand::Rng;
use std::error::Error;
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

pub struct RandomGenerator {
	sender: Sender<Message>,
	interval: Duration,
}

impl RandomGenerator {
	pub fn new(sender: Sender<Message>, interval_secs: u64) -> Self {
		RandomGenerator { sender, interval: Duration::from_secs(interval_secs) }
	}

	pub async fn start(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
		loop {
			sleep(self.interval).await;

			let num_bytes = rand::thread_rng().gen_range(1..=256);
			let random_bytes: Vec<u8> = (0..num_bytes).map(|_| rand::random::<u8>()).collect();

			self.sender.send(Message::Sign(Bytes(random_bytes))).await?;
		}
	}
}
