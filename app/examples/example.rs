use app::app::{App, Runnable};
use app::Service;
use async_trait::async_trait;
use error_registry::generated_errors::{Critical, GeneratedError};
use error_registry::BaseError;

use app::prelude::DependencyContainerParameter;
use healthchecker::{Alivable, HealthChecker, HealthcheckerServer};
use realis_macros::Alivable;
use schemas::{Agent, AuthInfo, Request, Schema};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use transport::{Response, VResponse};
use transport::{StanTransport, Transport};

const TOPIC_1: &str = "test-topic-1";
const TOPIC_2: &str = "test-topic-2";
const CLIENT_ID_1: &str = "test-client-1";
const CLIENT_ID_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";

const NATS_URL: &str = "localhost:4222";

const MESSAGES_NUMBER: i32 = 10;

#[tokio::main]
async fn main() {
    let dependency_container = DependencyContainer::new().await;

    let sender = Sender {
        transport: dependency_container.transport2.clone(),
    };

    App::new(dependency_container)
        .await
        .init_logger(None)
        .push_service::<ExampleService, RequestParamsSchema, ResponseSchema>()
        .await
        .unwrap()
        .push(sender)
        .run()
        .await;
}

#[derive(Clone)]
pub struct DependencyContainer<T: Transport + Send + Sync> {
    pub transport1: Arc<T>,
    pub transport2: Arc<T>,
    pub health_checker: HealthcheckerServer,
}

impl DependencyContainer<StanTransport> {
    pub async fn new() -> Self {
        let transport1 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_1)
            .expect("Fail to init transport_1");
        let transport2 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_2)
            .expect("Fail to init transport_2");
        let health_checker = HealthcheckerServer::new(&"127.0.0.1:8080".to_owned(), None)
            .await
            .expect("Fail to init health_checker");
        Self {
            transport1: Arc::new(transport1),
            transport2: Arc::new(transport2),
            health_checker,
        }
    }
}

impl<T: Transport + Send + Sync + Clone> DependencyContainerParameter<T>
    for DependencyContainer<T>
{
    fn get_transport(&self) -> Arc<T> {
        self.transport1.clone()
    }

    fn get_health_checker(&self) -> HealthChecker {
        self.health_checker.get_health_cheker()
    }
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

#[derive(Alivable, Clone)]
struct ExampleService {
    #[AliveAttr(skip)]
    a: i32,
    transport: Arc<StanTransport>,
}

impl From<DependencyContainer<StanTransport>> for ExampleService {
    fn from(dependency: DependencyContainer<StanTransport>) -> Self {
        Self {
            a: 1,
            transport: dependency.transport1.into(),
        }
    }
}

#[async_trait]
impl Service<RequestParamsSchema, ResponseSchema> for ExampleService {
    async fn process(
        &mut self,
        _request: Request<RequestParamsSchema>,
    ) -> Result<ResponseSchema, BaseError<Value>> {
        self.a += 1;

        if self.a == MESSAGES_NUMBER {
            return Err(BaseError::new(
                "".to_owned(),
                GeneratedError::Critical(Critical::Db).into(),
                None,
            ));
        }

        log::info!("{}", self.a);

        Ok(ResponseSchema {
            msg: "".to_string(),
        })
    }
}

// --- Runnable App ---

pub struct Sender {
    pub transport: Arc<StanTransport>,
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
                auth_info: Some(AuthInfo {
                    user_id: Some("".to_string()),
                    address: None,
                    continent: None,
                }),
            };

            let response = VResponse::Response(Response {
                topic_res: TOPIC_1.to_owned(),
                response: serde_json::to_vec(&schema).unwrap(),
            });
            let _result = self.transport.publish(response).await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
