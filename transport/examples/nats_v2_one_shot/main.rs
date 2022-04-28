use std::{thread, time::Duration};
use std::borrow::BorrowMut;
use std::mem::transmute;
use std::os::unix::raw::time_t;
use std::thread::sleep;
use error_registry::RealisErrors;
// use transport::nats_v2::Nats_v2;
// use transport::traits::MessageReceiver;
use async_trait::async_trait;
use nats_v2::asynk::Message;
use nats_v2::{ConnectInfo, Connection};
use serde_json::Value;
use serde_json::Value::String;
use error_registry::Nats::MessageReplyTimeout;
use transport::{
    nats_v2::{Nats_v2},
    traits::{MessageReceiver, Transport},
};

const TOPIC: &str = "test-nats2-stream-topic";
const NATS_URL: &str = "tls://0.0.0.0:4222";
const CLIENT_ID: &str = "test-client";
const CLUSTER_ID: &str = "test-cluster";

#[tokio::main]
async fn main() {
    let ResponseHandler = tokio::spawn( async {
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

    let RequestHandler = tokio::spawn( async{ // Listener
        let nats = transport::nats_v2::Nats_v2::new("localhost:4222", "test-client");

        match nats {
            Ok(connect) => {
                println!("Request id {}", connect.client.client_id());
                let replyment = connect.message_reply(TOPIC, "Responce-topic", "Request message".as_bytes().to_vec(), None).await;
                let replyMsg = std::string::String::from_utf8_lossy(replyment.as_ref().unwrap().as_slice());
                println!("REPLY: {:#?}", replyMsg);

            }
            Err(err) => {
                println!("ERROR! {:#?}", err);
            }
        }

    });

    ResponseHandler.await;
    RequestHandler.await;


}
pub struct MessageHandler {
    pub client: transport::nats_v2::Nats_v2
}

#[async_trait]
impl MessageReceiver<Vec<u8>, nats_v2::Message, RealisErrors> for MessageHandler {
    async fn process(&self, message: Vec<u8>, message_id: nats_v2::Message) -> Result<bool, RealisErrors> {
        println!("Got message: {:?}", message_id.subject);
        let data = std::string::String::from_utf8_lossy(&message);
        println!("Got data: {:?}", data);
        self.client.publish("Responce-topic", "Response message".as_bytes().to_vec(), None).await;
        Ok(false)
    }
}
