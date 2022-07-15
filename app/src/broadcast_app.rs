use crate::app::Runnable;
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::HealthChecker;
use log::debug;
use schemas::{Agent, Schema};
use serde_json::Value;
use std::sync::Arc;
use transport::{ReceivedMessage, Subscription, Transport, VSubscription};

#[async_trait]
pub trait BroadcastService<P: Agent, G: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> &'static str {
        P::topic()
    }

    async fn process(&mut self, request: P) -> Result<G, BaseError<Value>>;
}

// TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct BroadcastApp<P: Agent, G: Schema, S: BroadcastService<P, G>> {
    service: S,
    subscription: VSubscription,
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<(P, G)>,
}

#[async_trait]
impl<P: Agent, G: Schema, S: BroadcastService<P, G>> Runnable for BroadcastApp<P, G, S> {
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick::<String>(None);
        }
    }
}

impl<P: Agent, G: Schema, S: BroadcastService<P, G>> BroadcastApp<P, G, S> {
    pub async fn new<N>(
        service: S,
        transport: Arc<N>,
        health_checker: HealthChecker,
    ) -> Result<Self, BaseError<Value>>
    where
        N: Transport + Sync + Send,
    {
        transport
            .subscribe(service.topic_to_subscribe())
            .await
            .map(|subscription| Self {
                service,
                subscription,
                health_checker,
                _marker: Default::default(),
            })
    }

    async fn run_internal(&mut self) -> Result<(), BaseError<Value>> {
        loop {
            let message = self.subscription.next().await?;
            let result = match message.deserialize() {
                Ok(request) => match self.service.process(request).await {
                    Ok(response_schema) => {
                        debug!("got response schema{:#?}", response_schema);
                        Ok(())
                    }
                    Err(error) if error.is_critical() => {
                        log::debug!("Got response error critical: {:#?}", error);
                        Err(error)
                    }
                    Err(error) => {
                        log::debug!("Got response left: {:#?}", error);
                        Ok(())
                    }
                },
                Err(error) => {
                    debug!("got error{:#?}", error);
                    Ok(())
                }
            };
            result?;
            message.ok().await?;
        }
    }
}
