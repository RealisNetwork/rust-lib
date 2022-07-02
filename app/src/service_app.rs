use crate::app::Runnable;
use crate::service::Service;
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use schemas::{Response, ResponseMessage, ResponseResult, Schema};
use transport::{ReceivedMessage, Subscription, Transport, VReceivedMessage, VResponse, VSubscription, VTransport};
use transport::Response as TransportResponse;


// TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct ServiceApp<T: Schema, G: Schema, S: Service<T, G>, N: Transport + Sync + Send>
{
    service: S,
    transport: Arc<N>,
    subscription: VSubscription,
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<(T, G)>,
}

#[async_trait]
impl<T: Schema, G: Schema, S: Service<T, G>, N: Transport + Sync + Send> Runnable
    for ServiceApp<T, G, S, N>
{
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick();
        }
    }
}

impl<T: Schema, G: Schema, S: Service<T, G>, N: Transport + Sync + Send>
    ServiceApp<T, G, S, N>
{
    pub async fn new(
        service: S,
        mut transport: Arc<N>,
        health_checker: HealthChecker,
    ) -> Result<Self, BaseError<Value>> {
        transport
            .subscribe(&service.topic_to_subscribe())
            .await
            .map(|subscription| Self {
                service,
                transport,
                subscription,
                health_checker,
                _marker: Default::default(),
            })
    }

    async fn run_internal(&mut self) -> Result<(), BaseError<Value>> {
        loop {
            let message = self.subscription.next().await?;
            let raw_request: Value = message.deserialize()?;
            match message.deserialize() {
                Ok(request) => {
                    let response_schema = self.service.process(request).await?;
                    message.ok().await?;

                    let topic = "test".to_owned(); // TODO: fix me
                    let response: Response<_, _, ()> = Response {
                        result: ResponseResult {
                            request: raw_request,
                            response: ResponseMessage::Right { value: response_schema },
                        }
                    };
                    let payload = serde_json::to_vec(&response)
                        .unwrap(); // TODO: handle this

                    // TODO: rewrite publish raw_publish / publish
                    self.transport.publish(
                            VResponse::Response(
                                TransportResponse {
                                    topic_res: topic,
                                    response: payload
                                }
                            )
                        )
                        .await?;
                }
                Err(error) => {
                    self.on_deserialize_fail(message).await;
                    return Err(error);
                }
            }
        }
    }

    async fn on_deserialize_fail(&self, message: VReceivedMessage) {
        let _res = message.ok().await;
        // TODO: try get raw data, search for topicResponse/topicRes... publish response
    }
}
