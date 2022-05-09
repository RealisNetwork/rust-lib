use async_trait::async_trait;
use error_registry::RealisErrors;
use ratsio::StanMessage;
use futures::StreamExt;
use ratsio::{StanClient, StanOptions, StartPosition, RatsioError};
use transport::{
    nats::Nats,
    traits::{MessageReceiver, Transport},
};

const TOPIC: &str = "test-nust-stream-topic";
const CLIENT_ID: &str = "test_nuts";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";


#[tokio::main]
async fn main() {
    let opts = StanOptions::with_options(NATS_URL, CLUSTER_ID, CLIENT_ID);
    let stan_client = StanClient::from_options(opts).await.expect("Stan client error");
    loop {
        let mut message = String::new();
        std::io::stdin().read_line(&mut message).unwrap();
        println!("{}", &message);
        match stan_client.publish(TOPIC, &serde_json::to_vec(&message).unwrap()).await {
            Ok(()) => println!("Published with topic '{}' ", TOPIC),
            Err(_) => println!("Error "),
        };
    }
}