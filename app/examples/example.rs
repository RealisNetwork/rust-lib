use app::app::{App, Runnable};
use app::{Service, ServiceApp};
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::Alivable;
use healthchecker::HealthcheckerServer;
use log::LevelFilter;
use nats;
use realis_macros::Alivable;
use schemas::{Agent, AuthInfo, Request, Schema};
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
const NATS_URL: &str = "localhost:4222";

#[tokio::main]
async fn main() {
    let mut stan_transport_1 =
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_1).expect("Fail to init transport_1");

    let mut transport_1 = Arc::new(VTransport::Stan(stan_transport_1));
    let mut transport_2 =
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_2).expect("Fail to init transport_2");
    let service = ExampleService {
        a: 1,
        transport: transport_1.clone(),
    };
    let mut health_checker = HealthcheckerServer::new(&"127.0.0.1:8080".to_owned(), None)
        .await
        .expect("Fail to init health_checker");

    let service_app: ServiceApp<RequestSchema, ResponseSchema, ExampleService, VTransport> =
        ServiceApp::new(
            service.clone(),
            transport_1.clone(),
            health_checker.get_health_cheker(),
        )
        .await
        .expect("Fail to subscribe");

    health_checker.push(Box::new(service)).await;

    let sender = Sender {
        transport: transport_2.into(),
    };

    App::default()
        .init_logger_with_level(LevelFilter::Info)
        .push(service_app)
        .push(sender)
        .run()
        .await;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestSchema {
    msg: String,
}

impl Schema for RequestSchema {}

impl Agent for RequestSchema {
    fn topic() -> &'static str {
        TOPIC_1
    }

    fn method() -> &'static str {
        "todo!()"
    }

    fn agent() -> &'static str {
        "todo!()"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseSchema {
    msg: String,
}

impl Schema for ResponseSchema {}

#[derive(Alivable, Clone)]
struct ExampleService {
    #[AliveAttr(skip)]
    a: i32,
    transport: Arc<VTransport>,
}

#[async_trait]
impl Service<RequestSchema, ResponseSchema> for ExampleService {
    async fn process(
        &mut self,
        request: Request<RequestSchema>,
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
            let schema = Request::<ResponseSchema> {
                id: "".to_string(),
                topic_res: TOPIC_2.to_string(),
                agent: None,
                method: None,
                params: ResponseSchema {
                    msg: format!("{}", i),
                },
                auth: None,
                auth_info: AuthInfo {
                    user_id: "".to_string(),
                    address: None,
                    continent: None,
                },
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
