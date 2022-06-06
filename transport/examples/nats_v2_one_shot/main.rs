use std::{borrow::BorrowMut, mem::transmute, os::unix::raw::time_t, thread, thread::sleep, time::Duration};
// use transport::nats_v2::Nats_v2;
// use transport::traits::MessageReceiver;
use async_trait::async_trait;
use error_registry::BaseError;
use nats_not_async::{asynk::Message, ConnectInfo, Connection};
use serde_json::{Value, Value::String};
use transport::{
    nats_v2::Nats_v2,
    traits::{MessageReceiver, Transport},
};

const TOPIC: &str = "test-nats2-stream-topic";
const NATS_URL: &str = "tls://0.0.0.0:4222";
const CLIENT_ID: &str = "test-client";
const CLUSTER_ID: &str = "test-cluster";

#[tokio::main]
async fn main() {
    let response_handler = tokio::spawn(async {
        let nats = transport::nats_v2::Nats_v2::new("localhost:4222", "test-client");

        match nats {
            Ok(connect) => {
                println!("Response id {}", connect.client.client_id());
                let message_handler = MessageHandler { client: connect.clone() };
                let rsub = connect.subscribe(TOPIC, message_handler).await;
            }
            Err(err) => {
                println!("ERROR! {:#?}", err);
            }
        }
    });

    let request_handler = tokio::spawn(async {
        // Listener
        let nats = transport::nats_v2::Nats_v2::new("localhost:4222", "test-client");

        match nats {
            Ok(connect) => {
                println!("Request id {}", connect.client.client_id());
                let replyment = connect
                    .message_reply(TOPIC, "Responce-topic", "Request message".as_bytes().to_vec(), None)
                    .await;
                let reply_msg = std::string::String::from_utf8_lossy(replyment.as_ref().unwrap().as_slice());
                println!("REPLY: {:#?}", reply_msg);
            }
            Err(err) => {
                println!("ERROR! {:#?}", err);
            }
        }
    });

    request_handler.await;
    response_handler.await;
}
pub struct MessageHandler {
    pub client: transport::nats_v2::Nats_v2,
}

#[async_trait]
impl MessageReceiver<Vec<u8>, nats_not_async::Message, BaseError<()>> for MessageHandler {
    async fn process(&self, message: Vec<u8>, message_id: nats_not_async::Message) -> Result<bool, BaseError<()>> {
        println!("Got message: {:?}", message_id.subject);
        let data = std::string::String::from_utf8_lossy(&message);
        println!("Got data: {:?}", data);
        self.client
            .publish("Responce-topic", "Response message".as_bytes().to_vec(), None)
            .await;
        Ok(false)
    }
}
