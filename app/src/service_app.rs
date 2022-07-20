use crate::app::{AsyncTryFrom, DependencyContainerParameter, Runnable};
use crate::service::Service;
use async_trait::async_trait;
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use healthchecker::HealthChecker;
use schemas::{Agent, Response, ResponseMessage, ResponseResult, Schema};
use serde_json::Value;
use std::sync::Arc;
use transport::{ReceivedMessage, Subscription, Transport, VReceivedMessage, VSubscription};

//TODO: ServiceAppBuilder|ServiceAppContainer?
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
            health_checker.make_sick::<String>(None);
        }
        let _result = self.after_run().await;
    }
}

#[async_trait]
impl<T, P, G, ServiceInner, N> AsyncTryFrom<Arc<T>> for ServiceApp<P, G, ServiceInner, N>
where
    T: 'static + Clone + Send + Sync + DependencyContainerParameter<N>,
    P: Agent,
    G: Schema,
    ServiceInner: 'static + From<Arc<T>> + Service<P, G>,
    N: 'static + Transport + Sync + Send,
{
    type Error = BaseError<Value>;

    async fn async_try_from(dependency_container: Arc<T>) -> Result<Self, BaseError<Value>> {
        ServiceApp::new(
            ServiceInner::from(dependency_container.clone()),
            dependency_container.get_transport(),
            dependency_container.get_health_checker(),
        )
        .await
    }
}

impl<P: Agent, G: Schema, S: Service<P, G>, N: Transport + Sync + Send> ServiceApp<P, G, S, N> {
    pub async fn new(
        service: S,
        transport: Arc<N>,
        health_checker: HealthChecker,
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
            "Run service: \nagent: \t{:?},\nmethod:\t{:?},\ntopic: \t{:?},",
            P::agent(),
            P::method(),
            P::topic(),
        );
        // Notification gateway
        self.run_notification().await?;

        Ok(())
    }

    async fn after_run(&mut self) -> Result<(), BaseError<Value>> {
        // Logs
        log::warn!(
            "Stop service: \nagent: \t{:?},\nmethod:\t{:?},\ntopic: \t{:?},",
            P::agent(),
            P::method(),
            P::topic(),
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
                "topic": P::topic(),
                "paramsSchema": P::schema(),
                "responseSchema": G::schema(),
            }
        });

        self.transport
            .raw_publish(TOPIC.to_owned(), &notification)
            .await?;

        Ok(())
    }

    async fn run_internal(&mut self) -> Result<(), BaseError<Value>> {
        self.before_run().await?;
        let topic = P::topic();

        loop {
            let message = self.subscription.next().await?;
            match message.deserialize() {
                Ok(request) => {
                    log::info!("By topic: {:?} | Got request: {:#?}", topic, request);
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
        log::debug!(
            "Preparing schema: {:#?} for publish by topic: {}",
            response,
            topic
        );

        self.transport.raw_publish(topic.clone(), &response).await?;

        log::info!("By topic: {:?} | Publish {:#?}", topic, response);
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
