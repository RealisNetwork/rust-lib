use nats;
use serde_json::Value;
use app::{Service, ServiceApp};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use transport::response::{Response, VResponse};
use transport::transport::StanTransport;
use serde::Deserialize;
use async_trait::async_trait;
use transport::Transport;

const TOPIC: &str = "test-topic";
const CLIENT_ID: &str = "test-client";
const CLUSTER_ID: &str = "nats-streaming";
const NATS_URL: &str = "127.0.0.1:4222";

#[tokio::main]
async fn main() {
    let mut transport = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID)
        .expect("Fail to init transport");
    let service = SchemaService;
    let health_checker = HealthChecker::new(&"127.0.0.1:4444".to_owned(), 1_000)
        .await
        .expect("Fail to init health_checker");

    // transport.publish(VResponse::Response(Response {
    //     topic_res: TOPIC.to_owned(),
    //     response: serde_json::to_vec(&serde_json::json!({
    //         "msgs": "test fail"
    //     })).expect("Fail to serialise to bytes")
    // })).await.expect("Fail to publish test message");

    ServiceApp::new(
        service,
        transport.into(),
        health_checker
    )
        .run()
        .await;
}

#[derive(Debug, Deserialize)]
struct Schema {
    msg: String,
}

struct SchemaService;

#[async_trait]
impl Service<Schema> for SchemaService {
    fn topic_to_subscribe(&self) -> String {
        TOPIC.to_owned()
    }

    async fn process(&mut self, schema: Schema) -> Result<Vec<VResponse>, BaseError<Value>> {
        println!("{:#?}", schema);
        Ok(vec![])
    }
}
