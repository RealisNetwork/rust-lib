use crate::app::Runnable;
use crate::service::Service;
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;
use transport::{
    ReceivedMessage, Subscription, Transport, VReceivedMessage, VSubscription, VTransport,
};

// TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct ServiceApp<T: DeserializeOwned + Send + Sync, S: Service<T>, N: Transport + Sync + Send> {
    service: S,
    transport: N,       // TODO: use generic type `T: Transport`
    subscription: VSubscription, // TODO: use generic type `_: Subscription`
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<T>,
}

#[async_trait]
impl<T: DeserializeOwned + Send + Sync, S: Service<T>, N: Transport + Sync + Send> Runnable for ServiceApp<T, S, N> {
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick();
        }
    }
}

impl<T: DeserializeOwned + Send + Sync, S: Service<T>, N: Transport + Sync + Send> ServiceApp<T, S, N> {
    pub async fn new(
        service: S,
        mut transport: N,
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
                Ok(request) => {
                    let result = self.service.process(request).await?;
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
