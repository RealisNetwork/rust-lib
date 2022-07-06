use crate::app::Runnable;
use crate::service::Service;
use async_trait::async_trait;
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use schemas::{Agent, Response, ResponseMessage, ResponseResult, Schema};
use serde_json::Value;
use std::sync::Arc;
use transport::Response as TransportResponse;
use transport::{
    ReceivedMessage, Subscription, Transport, VReceivedMessage, VResponse, VSubscription,
};

// TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct ServiceApp<P: Agent, G: Schema, S: Service<P, G>, N: Transport + Sync + Send> {
    service: S,
    transport: Arc<N>,
    subscription: VSubscription,
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<(P, G)>,
}

#[async_trait]
impl<P: Agent, G: Schema, S: Service<P, G>, N: Transport + Sync + Send> Runnable
    for ServiceApp<P, G, S, N>
{
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick();
        }
    }
}

impl<P: Agent, G: Schema, S: Service<P, G>, N: Transport + Sync + Send> ServiceApp<P, G, S, N> {
    pub async fn new(
        service: S,
        transport: Arc<N>,
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
            match message.deserialize() {
                Ok(request) => match self.service.process(request).await {
                    Ok(response_schema) => {
                        self.on_process_success(message, response_schema).await?
                    }
                    Err(error) if error.is_critical() => return Err(error),
                    Err(error) => self.on_process_error(message, error).await?,
                },
                Err(error) => {
                    self.on_process_error(message, error.clone()).await?;
                    return Err(error);
                }
            }
        }
    }

    async fn on_process_success(
        &self,
        message: VReceivedMessage,
        response: G,
    ) -> Result<(), BaseError<Value>> {
        let response = ResponseMessage::Right { value: response };
        self.process_response(message, response).await
    }

    async fn on_process_error(
        &self,
        message: VReceivedMessage,
        error: BaseError<Value>,
    ) -> Result<(), BaseError<Value>> {
        let response = ResponseMessage::Left { value: error };
        self.process_response(message, response).await
    }

    /// Ack message, then try get topic from request,
    /// create response from request and provided response
    /// and publish this response
    /// If one of the steps failed return `Err`
    /// otherwise `Ok`
    async fn process_response(
        &self,
        message: VReceivedMessage,
        response: ResponseMessage<G, Value>,
    ) -> Result<(), BaseError<Value>> {
        let raw_request: Value = message.deserialize()?;
        message.ok().await?;
        let topic = Self::get_topic_response(&raw_request)?;
        let response = Response {
            result: ResponseResult {
                request: &raw_request,
                response,
            },
        };
        let payload = serde_json::to_vec(&response).map_err(|error| {
            BaseError::new(
                format!("{:?}", error),
                GeneratedError::Common(Common::InternalServerError).into(),
                serde_json::to_value(&raw_request).ok(),
            )
        })?;

        self.transport
            .publish(VResponse::Response(TransportResponse {
                topic_res: topic,
                response: payload,
            }))
            .await
    }

    /// Try get topic to response from raw request
    /// return `OK` if find topic in one of such fields "topicResponse"
    /// otherwise return `Err`
    fn get_topic_response(request: &Value) -> Result<String, BaseError<Value>> {
        match request.get("topicResponse") {
            None => Err(GeneratedError::Common(Common::InternalServerError).into()),
            Some(topic) => Ok(topic.to_string()),
        }
    }
}
