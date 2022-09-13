use crate::app::Runnable;
use async_trait::async_trait;
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use healthchecker::Healthchecker;
use schemas::{Agent, Request, Response, ResponseMessage, ResponseResult, Schema};
use serde_json::json;
use serde_json::Value;
use std::sync::Arc;
use transport::{ReceivedMessage, Subscription, Transport, VReceivedMessage, VSubscription};

#[async_trait]
pub trait Service<Params: Agent, Returns: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> &'static str {
        Params::topic()
    }

    async fn process(&mut self, request: Request<Params>) -> Result<Returns, BaseError<Value>>;
}

//TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct ServiceApp<Params: Agent, Returns: Schema, S: Service<Params, Returns>, T: Transport> {
    service: S,
    transport: Arc<T>,
    subscription: VSubscription,
    health_checker: Healthchecker,
    _marker: std::marker::PhantomData<(Params, Returns)>,
}

#[async_trait]
impl<Params: Agent, Returns: Schema, S: Service<Params, Returns>, T: Transport> Runnable
    for ServiceApp<Params, Returns, S, T>
{
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            health_checker.make_sick(Some(error));
        }
        let _result = self.after_run().await;
    }
}

impl<Params: Agent, Returns: Schema, S: Service<Params, Returns>, T: Transport>
    ServiceApp<Params, Returns, S, T>
{
    pub async fn new(
        service: S,
        transport: Arc<T>,
        health_checker: Healthchecker,
    ) -> Result<Self, BaseError<Value>> {
        transport
            .subscribe(service.topic_to_subscribe())
            .await
            .map(|subscription| Self {
                service,
                transport,
                subscription,
                health_checker,
                _marker: Default::default(),
            })
    }

    async fn before_run(&mut self) -> Result<(), BaseError<Value>> {
        // Logs
        log::info!(
            "{}",
            json!({
                "Run service": {
                    "agent": Params::agent(),
                    "method": Params::method(),
                    "topic": Params::topic(),
                }
            })
            .to_string()
        );
        // Notification gateway
        self.run_notification().await?;

        Ok(())
    }

    async fn after_run(&mut self) -> Result<(), BaseError<Value>> {
        // Logs
        log::warn!(
            "{}",
            json!({
                "Stop service": {
                    "agent": Params::agent(),
                    "method": Params::method(),
                    "topic": Params::topic(),
                }
            })
            .to_string()
        );

        Ok(())
    }

    /// Send notification to gateway in JSON format:
    /// {
    ///   "name": "string",
    ///   "client_id": "string",
    ///   "schemas": {
    ///     "topic": "string"
    ///     "paramsSchema": {}
    ///     "responseSchema": {}
    ///   }
    /// }
    async fn run_notification(&mut self) -> Result<(), BaseError<Value>> {
        const TOPIC: &str = "pasha_help_plz";
        let notification = serde_json::json!({
            "schemas": {
                "topic": Params::topic(),
                "paramsSchema": Params::schema(),
                "responseSchema": Returns::schema(),
            }
        });

        self.transport
            .raw_publish(TOPIC.to_owned(), &notification)
            .await?;

        Ok(())
    }

    async fn run_internal(&mut self) -> Result<(), BaseError<Value>> {
        self.before_run().await?;
        let topic = Params::topic();

        loop {
            let message = self.subscription.next().await?;
            match message.deserialize() {
                Ok(request) => {
                    log::info!(
                        "{}",
                        json!({
                            "topic": topic,
                            "request": request
                        })
                        .to_string()
                    );
                    match self.service.process(request).await {
                        Ok(response_schema) => {
                            log::debug!("got response schema{:#?}", response_schema);
                            self.on_process_success(message, response_schema).await?
                        }
                        Err(error) if error.is_critical() => {
                            log::debug!("Got response error critical: {:#?}", error);
                            return Err(error);
                        }
                        Err(error) => {
                            log::debug!("Got response left: {:#?}", error);
                            self.on_process_error(message, error).await?
                        }
                    }
                }
                Err(error) => {
                    log::debug!("got error{:#?}", error);
                    self.on_process_error(message, error.clone()).await?;
                    return Err(error);
                }
            }
        }
    }

    async fn on_process_success(
        &self,
        message: VReceivedMessage,
        response: Returns,
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
        response: ResponseMessage<Returns, Value>,
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
        log::debug!(
            "Preparing schema: {:#?} for publish by topic: {}",
            response,
            topic
        );

        self.transport.raw_publish(topic.clone(), &response).await?;

        log::info!("By topic: {:?} | Publish {:?}", topic, response);
        Ok(())
    }

    /// Try get topic to response from raw request
    /// return `OK` if find topic in one of such fields "topicResponse"
    /// otherwise return `Err`
    fn get_topic_response(request: &Value) -> Result<String, BaseError<Value>> {
        let result = match request.get("topicResponse") {
            None => Err(GeneratedError::Common(Common::InternalServerError).into()),

            Some(topic) => Ok(topic
                .as_str()
                .ok_or_else(|| {
                    BaseError::<Value>::new(
                        "Unexpected type".to_string(),
                        GeneratedError::Common(Common::Unknown).into(),
                        None,
                    )
                })?
                .to_string()),
        };
        log::debug!("request {:#?} : ", request);
        result
    }
}
