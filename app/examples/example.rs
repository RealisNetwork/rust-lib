use app::app::{App, Runnable};
use app::{Service, ServiceApp};
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use nats;
use schemas::Schema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use transport::{Response, VResponse};
use transport::{StanTransport, Transport, VTransport};

const TOPIC_1: &str = "test-topic";
const TOPIC_2: &str = "test-topic-2";
const CLIENT_ID_1: &str = "test-client-1";
const CLIENT_ID_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";

#[tokio::main]
async fn main() {
    let mut transport_1 = Arc::new(VTransport::Stan(
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_1).expect("Fail to init transport_1"),
    ));
    let mut transport_2 =
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_2).expect("Fail to init transport_2");
    let service = ExampleService;
    let health_checker = HealthChecker::new(&"127.0.0.1:4444".to_owned(), 1_000)
        .await
        .expect("Fail to init health_checker");

    let service_app: ServiceApp<RequestSchema, ResponseSchema, ExampleService, VTransport> =
        ServiceApp::new(service, transport_1, health_checker)
            .await
            .expect("Fail to subscribe");

    let sender = Sender {
        transport: transport_2.into(),
    };

    App::default().push(service_app).push(sender).run().await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestSchema {
    msg: String,
}

impl Schema for RequestSchema {}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseSchema {
    msg: String,
}

impl Schema for ResponseSchema {}

struct ExampleService;

#[async_trait]
impl Service<RequestSchema, ResponseSchema> for ExampleService {
    fn topic_to_subscribe(&self) -> String {
        TOPIC_1.to_owned()
    }

    async fn process(
        &mut self,
        request: RequestSchema,
    ) -> Result<ResponseSchema, BaseError<Value>> {
        println!("{:#?}", request);
        Ok(ResponseSchema {
            msg: "".to_string(),
        })
    }
}

pub struct Sender {
    pub transport: VTransport,
}

#[async_trait]
impl Runnable for Sender {
    async fn run(&mut self) {
        for i in 0..10 {
            let schema = ResponseSchema {
                msg: format!("{}", i),
            };

            let response = VResponse::Response(Response {
                topic_res: TOPIC_1.to_owned(),
                response: serde_json::to_vec(&schema).unwrap(),
            });
            self.transport.publish(response).await;
            println!("publish");

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
