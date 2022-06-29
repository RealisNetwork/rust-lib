use crate::service::Service;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use serde::de::DeserializeOwned;
use serde_json::Value;
use transport::{ReceivedMessage, Subscription, Transport, VReceivedMessage, VSubscription, VTransport};

pub struct ServiceApp<T: DeserializeOwned, S: Service<T>> {
    service: S,
    transport: VTransport, // TODO: use generic type `T: Transport`
    subscription: VSubscription, // TODO: use generic type `_: Subscription`
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned, S: Service<T>> ServiceApp<T, S> {
    pub async fn new(service: S, mut transport: VTransport, health_checker: HealthChecker) -> Result<Self, BaseError<Value>> {
        transport
            .subscribe(&service.topic_to_subscribe())
            .await
            .map(|subscription| Self {
                service,
                transport,
                subscription,
                health_checker,
                _marker: Default::default()
            })
    }
    
    pub async fn run(self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick();
        }
    }

    async fn run_internal(mut self) -> Result<(), BaseError<Value>> {
        loop {
            let message = self.subscription.next().await?;
            match message.deserialize() {
                Ok(schema) => {
                    let result = self.service.process(schema).await?;
                    message.ok().await?;

                    for response in result {
                        self.transport.publish(response).await?
                    }
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
