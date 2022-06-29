use nats;
use serde_json::Value;
use app::{Service, ServiceApp};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use transport::{Response, VResponse};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use app::app::{App, Runnable};
use transport::{Transport, VTransport, StanTransport};

const TOPIC_1: &str = "test-topic";
const TOPIC_2: &str = "test-topic-2";
const CLIENT_ID_1: &str = "test-client-1";
const CLIENT_ID_2: &str = "test-client-2";
const CLUSTER_ID: &str = "nats-streaming";
const NATS_URL: &str = "127.0.0.1:4222";

#[tokio::main]
async fn main() {
    let mut transport_1 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_1)
        .expect("Fail to init transport_1");
    let mut transport_2 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_2)
        .expect("Fail to init transport_2");
    let service = SchemaService;
    let health_checker = HealthChecker::new(&"127.0.0.1:4444".to_owned(), 1_000)
        .await
        .expect("Fail to init health_checker");



    let service_app = ServiceApp::new(
        service,
        transport_1.into(),
        health_checker
    ).await.expect("Fail to subscribe");

    let sender = Sender {
        transport: transport_2.into()
    };

    App::default()
        .push(service_app)
        .push(sender)
        .run()
        .await;
}

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    msg: String,
}

struct SchemaService;

#[async_trait]
impl Service<Schema> for SchemaService {
    fn topic_to_subscribe(&self) -> String {
        TOPIC_1.to_owned()
    }

    async fn process(&mut self, schema: Schema) -> Result<Vec<VResponse>, BaseError<Value>> {
        println!("{:#?}", schema);
        Ok(vec![])
    }
}

pub struct Sender {
    pub transport: VTransport
}

#[async_trait]
impl Runnable for Sender {
    async fn run(&mut self) {
        for i in 0..10 {
            let schema = Schema { msg: format!("{}", i) };

            let response = VResponse::Response(Response {
                    topic_res: TOPIC_1.to_owned(),
                    response: serde_json::to_vec(&schema).unwrap()
            });
            self.transport.publish(response).await;
            println!("publish");

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}