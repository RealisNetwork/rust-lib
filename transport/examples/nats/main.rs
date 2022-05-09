
use std::sync::Arc;
use async_trait::async_trait;
use error_registry::{Nats as NatsError, RealisErrors};
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

    // Subscription. Step 1 
    let secs: i32 = 1;
    let (stan_id, mut stream) = stan_client
        .subscribe_with_all(TOPIC, None, None, 1024, secs , StartPosition::NewOnly, 0, None, true)
        .await
        .expect("stan_client error");

    // Send message. Step 2
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();
    println!("{}", &message);
    match stan_client.publish(TOPIC, &serde_json::to_vec(&message).unwrap()).await {
        Ok(()) => println!("Published with topic '{}' ", TOPIC),
        Err(_) => println!("Error "),
    };

    let message_handler = MessageHandler {
        stan_client: stan_client.clone()
    };
    loop {
        match stream.next().await {
            None => {
                println!("Unsubscribe. #1");
                break;
            }
            Some(message) => match message_handler.process(message.payload.clone(), message).await {
                Ok(true) => {}
                Ok(false) => {
                    // Unsub. Step 5
                    stan_client
                        .un_subscribe(&stan_id)
                        .await
                        .map_err(|_| RealisErrors::Nats(NatsError::Unsubscribe));
                    println!("Unsubscribe. #1");
                    break;
                }
                Err(_) => {
                    // Unsub. Step 5
                    stan_client
                        .un_subscribe(&stan_id)
                        .await
                        .map_err(|_| RealisErrors::Nats(NatsError::Unsubscribe));
                    println!("Unsubscribe. #1");
                    break;
                }
            },
        }
    }

    // Subscription. Step 6
    let (stan_id, mut stream) = stan_client
        .subscribe_with_all(TOPIC, None, None, 1024, secs , StartPosition::NewOnly, 0, None, true)
        .await
        .expect("stan_client error");

    // Expect no Ok messages. Step 7
    loop {
        match stream.next().await {
            None => {
                println!("Empty! Unsubscribe. #1");
                break;
            }
            Some(message) => match message_handler.process(message.payload.clone(), message).await {
                Ok(true) => {}
                Ok(false) => {
                    // Unsub. Step 5
                    println!("Unsubscribe. #1");
                    break;
                }
                Err(_) => {
                    // Unsub. Step 5
                    println!("Unsubscribe. #1");
                    break;
                }
            },
        }
    }
}


pub struct MessageHandler {
    stan_client: Arc<StanClient>
}

#[async_trait]
impl MessageReceiver<Vec<u8>, StanMessage, RealisErrors> for MessageHandler {
    async fn process(&self, message: Vec<u8>, message_id: StanMessage) -> Result<bool, RealisErrors> {
        // Recive. Step 3
        // println!("Got message: {:#?}", message_id);

        let data = serde_json::from_slice::<String>(&message).unwrap();
        println!("Got data: {:#?}", &data);
        // Ok message. Step 4
        self.stan_client
            .acknowledge(message_id)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Ok));

        Ok(false)
    }
}
