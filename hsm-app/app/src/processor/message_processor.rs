use crate::hsm::hashicorp::Hsm;
use crate::types::types::{Bytes, Message};
use futures_util::StreamExt;
use log::{error, info};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
pub struct MessageProcessor<H>
where
	H: Hsm + Send + Sync + 'static,
{
	hsm: Arc<H>,
	receiver: Receiver<Message>,
	worker_count: usize,
}

impl<H> MessageProcessor<H>
where
	H: Hsm + Send + Sync + 'static,
{
	pub fn new(hsm: H, receiver: Receiver<Message>, worker_count: usize) -> Self {
		MessageProcessor { hsm: Arc::new(hsm), receiver, worker_count }
	}

	pub async fn start(self) {
		info!(
			"MessageProcessor started with a concurrency limit of {} workers.",
			self.worker_count
		);

		let stream = ReceiverStream::new(self.receiver);

		stream
			.for_each_concurrent(self.worker_count, |message| {
				let hsm_clone = Arc::clone(&self.hsm);
				async move {
					match message {
						Message::Sign(bytes) => {
							if let Err(e) = handle_sign_message(&hsm_clone, bytes).await {
								error!("Error handling Sign message: {}", e);
							}
						}
						Message::Verify(_, _) => {
							info!("Received unexpected Verify message. Ignoring.");
						}
					}
				}
			})
			.await;

		info!("MessageProcessor has terminated as all messages have been processed.");
	}
}

async fn handle_sign_message<H>(
	hsm: &Arc<H>,
	message: Bytes,
) -> Result<(), Box<dyn Error + Send + Sync>>
where
	H: Hsm + Send + Sync,
{
	let signature = hsm.sign(message.clone()).await?;
	println!("Signed message: {:?}", signature.0);

	let is_valid = hsm.verify(message, signature).await?;
	println!("Verified message: {:?}", is_valid);

	Ok(())
}
