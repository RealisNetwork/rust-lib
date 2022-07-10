use app::app::{App, LevelFilter, Runnable};
use app::{Service, ServiceApp};
use async_trait::async_trait;
use error_registry::generated_errors::{Critical, GeneratedError};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use schemas::{Agent, AuthInfo, Request, Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use transport::{Response, VResponse};
use transport::{StanTransport, Transport, VTransport};

const NAME: &str = "example-0";
const TOPIC_1: &str = "test-topic";
const CLIENT_ID_1: &str = "test-client-1";
const CLIENT_ID_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";
const MESSAGES_NUMBER: u32 = 10;

#[tokio::main]
async fn main() {
    let transport_1 = Arc::new(VTransport::Stan(
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_1).expect("Fail to init transport_1"),
    ));
    let transport_2 =
        StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_2).expect("Fail to init transport_2");
    let service = ExampleService::default();
    let health_checker = HealthChecker::new(&"127.0.0.1:4444".to_owned(), 1_000)
        .await
        .expect("Fail to init health_checker");

    let service_app: ServiceApp<RequestParamsSchema, ResponseSchema, ExampleService, VTransport> =
        ServiceApp::new(NAME.to_owned(), CLIENT_ID_1.to_owned(), service, transport_1, health_checker)
            .await
            .expect("Fail to subscribe");

    let sender = Sender {
        transport: transport_2.into(),
    };

    App::default()
        .init_logger_with_level(LevelFilter::Debug)
        .push(service_app)
        .push(sender)
        .run()
        .await;
}

// --- RequestParamsSchema ---

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RequestParamsSchema {
    msg: String,
}

impl Schema for RequestParamsSchema {
    fn schema() -> Value {
        serde_json::json!({
            "msg": {
                "type": "string"
            }
        })
    }
}

impl Agent for RequestParamsSchema {
    fn topic() -> &'static str {
        TOPIC_1
    }

    fn method() -> &'static str {
        "topic"
    }
    fn agent() -> &'static str {
        "test"
    }
}

// --- ResponseScheama ---

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseSchema {
    msg: String,
}

impl Schema for ResponseSchema {
    fn schema() -> Value {
        serde_json::json!({
            "msg": {
                "type": "string"
            }
        })
    }
}

// --- Service ---

struct ExampleService {
    counter: u32,
}

impl Default for ExampleService {
    fn default() -> Self {
        Self { counter: 0 }
    }
}

#[async_trait]
impl Service<RequestParamsSchema, ResponseSchema> for ExampleService {
    async fn process(
        &mut self,
        _request: Request<RequestParamsSchema>,
    ) -> Result<ResponseSchema, BaseError<Value>> {
        self.counter += 1;

        if self.counter == MESSAGES_NUMBER {
            return Err(BaseError::new(
                "".to_owned(),
                GeneratedError::Critical(Critical::Db).into(),
                None,
            ));
        }

        Ok(ResponseSchema {
            msg: "".to_string(),
        })
    }
}

// --- Runnable App ---

pub struct Sender {
    pub transport: VTransport,
}

#[async_trait]
impl Runnable for Sender {
    async fn run(&mut self) {
        for i in 0..MESSAGES_NUMBER {
            let request = Request {
                id: format!("id-{}", i),
                topic_res: format!("topic-{}", i),
                agent: None,
                method: None,

                params: RequestParamsSchema {
                    msg: format!("{}", i),
                },

                auth: None,
                auth_info: AuthInfo {
                    user_id: "".to_owned(),
                    address: None,
                    continent: None,
                },
            };

            let response = VResponse::Response(Response {
                topic_res: TOPIC_1.to_owned(),
                response: serde_json::to_vec(&request).unwrap(),
            });
            let _result = self.transport.publish(response).await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
