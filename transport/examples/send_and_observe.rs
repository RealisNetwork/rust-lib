extern crate core;
use app::app::{App, Runnable};
use app::{Service, ServiceApp};
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use transport::{
    ReceivedMessage, Response as ServiceResponse, StanTransport, VResponse, VTransport,
};
use transport::{Response, Transport};

const TOPIC_TEST_1: &str = "test-topic";
const TOPIC_TEST_2: &str = "test-topic-2";
const CLIENT_ID_TEST_1: &str = "test-client-1";
const CLIENT_ID_TEST_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    topic_response: String,
    msg: String,
}

pub struct Sender {
    pub transport: VTransport,
}

struct SchemaService;

#[async_trait]
impl Service<Schema> for SchemaService {
    fn topic_to_subscribe(&self) -> String {
        TOPIC_TEST_1.to_owned()
    }

    async fn process(&mut self, request: Schema) -> Result<Vec<VResponse>, BaseError<Value>> {
        println!("{:#?}", request);
        //tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        Ok(vec![ServiceResponse::new(
            request.topic_response.as_str(),
            serde_json::to_vec(&request).expect("Can't serialize"),
        )
        .into()])
    }
}

#[tokio::main]
pub async fn main() {
    let mut transport_1 = VTransport::Stan(
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_1)
            .expect("Fail to init transport_1"),
    );

    let mut transport_2 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_2)
        .expect("Fail to init transport_2");

    let service = SchemaService;

    let health_checker = HealthChecker::new(&"127.0.0.1:4444".to_owned(), 1_000)
        .await
        .expect("Fail to init health_checker");

    let service_app: ServiceApp<Schema, SchemaService, VTransport> =
        ServiceApp::new(service, transport_1.into(), health_checker)
            .await
            .expect("Fail to subscribe");

    let sender = Sender {
        transport: transport_2.into(),
    };

    App::default().push(service_app).push(sender).run().await;
}

#[async_trait]
impl Runnable for Sender {
    async fn run(&mut self) {
        for i in 0..5 {
            let schema = Schema {
                topic_response: "topic_to_wait".to_string(),
                msg: format!("{}", i),
            };

            let message = self
                .transport
                .send_message_and_observe_reply(
                    "topic_to_wait".to_string(),
                    VResponse::Response(Response::new(
                        TOPIC_TEST_1,
                        serde_json::to_vec(&schema).expect("Can't serialize"),
                    )),
                    Some(Duration::from_secs(5)),
                )
                .await
                .unwrap();

            let obtained_msg: Schema = message.deserialize().unwrap();

            message.ok().await;

            println!("obtained_msg: {:#?}", obtained_msg);

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
