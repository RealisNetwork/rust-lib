use error_registry::RealisErrors;
use std::{thread, thread::sleep, time::Duration};
// use transport::nats_v2::Nats_v2;
// use transport::traits::MessageReceiver;
use async_trait::async_trait;
use nats_v2::{asynk::Message, ConnectInfo, Connection};
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
    let LH = thread::spawn(move || {
        // Listener
        let connection: Connection;
        let nc = nats_v2::Options::new().with_name(CLIENT_ID).connect("nats://localhost:4222");
        match nc {
            Ok(conect) => {
                println!("Listener id {}", conect.client_id());
                let reply = conect.new_inbox();
                let rsub = conect.subscribe("my.subject").unwrap();

                for msg in rsub.messages() {
                    let response = std::string::String::from_utf8_lossy(&msg.data);
                    println!("MSG: {:#?} | {:#?}", msg.subject, response);
                }
                connection = conect;
            }
            Err(_) => {
                println!("ERROR!");
            }
        }
    });

    let PH = thread::spawn(move || {
        let nc = nats_v2::Options::new()
            .with_name(CLIENT_ID)
            .connect("nats://localhost:4222")
            .unwrap();
        println!("Publisher id {}", nc.client_id());
        nc.publish("my.subject", "uifehsf fhusiseh fhesiusufih fhesiuhifh1 1431");
        nc.publish("my.subject", "1283791 fhusiseh fhesiusufih fhesiuhifh1 1431");
        nc.publish("my.subject", "uifehsf  fhesiusufih fhesiuhifh1 1431");
    });

    PH.join();
    LH.join();
}
