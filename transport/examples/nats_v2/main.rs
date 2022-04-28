use std::{thread, time::Duration};
use error_registry::RealisErrors;
use async_trait::async_trait;
use nats_v2::asynk::Message;
use nats_v2::{ConnectInfo, Connection};
use serde_json::Value;
use serde_json::Value::String;
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
    let LH = tokio::spawn(async { // Listener
        let nats = transport::nats_v2::Nats_v2::new("localhost:4222", "test-client");

        match nats {
            Ok(conect) => {
                println!("Listener id {}", conect.client.client_id());

                let mut message_handler = MessageHandler{};
                let rsub = conect.subscribe("my.subject", message_handler).await;

            }
            Err(err) => {
                println!("ERROR! {:#?}", err);
            }
        }

    });
    std::thread::sleep(Duration::from_secs(2));
    let PH = tokio::spawn(async {
        let nc = nats_v2::Options::new().with_name(CLIENT_ID).connect("nats://localhost:4222").unwrap();
        println!("Publisher id {}", nc.client_id());
        for i in 0..200 {
            nc.publish("my.subject", format!("uifehsf fhusiseh fhesiusufih fhesiuhifh1 1431: {}", i));
            std::thread::sleep(Duration::from_millis(200));
        }
        nc.close();

    });

    LH.await;
    PH.await;



}

pub struct MessageHandler {

}

#[async_trait]
impl MessageReceiver<Vec<u8>, nats_v2::Message, RealisErrors> for MessageHandler {
    async fn process(&self, message: Vec<u8>, message_id: nats_v2::Message) -> Result<bool, RealisErrors> {
        println!("Got message: {:?}", message_id.subject);
        let data = std::string::String::from_utf8_lossy(&message);
        println!("Got data: {:?}", data);
        Ok(true)
    }
}