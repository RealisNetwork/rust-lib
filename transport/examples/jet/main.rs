use transport::jet::{Jet, jet_nats::Message};
use transport::traits::{MessageReceiver, Transport};
use async_trait::async_trait;
use error_registry::RealisErrors;

const TOPIC: &str = "test-jet-stream-topic";
const NATS_URL: &str = "127.0.0.1:4222";

#[tokio::main]
async fn main() {
	let transport = Jet::new(NATS_URL).await;

	let message = "Hello, World!";

	match transport.publish(TOPIC, serde_json::to_vec(message).unwrap(), None).await {
		Ok(()) => {
			println!("Publish by topic: `{}`, message: `{:?}`", TOPIC, message);

			let message_handler = MessageHandler {};

			let _res = transport.subscribe(TOPIC, message_handler).await;
		}
		Err(error) => println!("Fail to publish: {:?}", error),
	}
}

pub struct MessageHandler {}

#[async_trait]
impl MessageReceiver<Vec<u8>, Message, RealisErrors> for MessageHandler
{
	async fn process(&self, message: Vec<u8>, message_id: Message) -> Result<bool, RealisErrors> {
		println!("Got message: {:?}", message_id);

		let data = serde_json::from_slice::<String>(&message).unwrap();
		println!("Got data: {:?}", data);

		Ok(false)
	}
}