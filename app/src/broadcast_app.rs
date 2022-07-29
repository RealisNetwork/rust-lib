use crate::app::Runnable;
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::Healthchecker;
use schemas::{Agent, Schema};
use serde_json::Value;
use std::sync::Arc;
use transport::{ReceivedMessage, Subscription, Transport, VSubscription};

#[async_trait]
pub trait BroadcastService<Params: Agent, Returns: Schema>: Send + Sync {
    fn topic_to_subscribe(&self) -> &'static str {
        Params::topic()
    }

    async fn process(&mut self, request: Params) -> Result<Returns, BaseError<Value>>;
}

// TODO: ServiceAppBuilder|ServiceAppContainer?
pub struct BroadcastApp<
    Params: Agent,
    Returns: Schema,
    S: BroadcastService<Params, Returns>,
    N: Transport + Sync + Send,
> {
    service: S,
    subscription: VSubscription,
    health_checker: Healthchecker,
    _marker: std::marker::PhantomData<(Params, Returns, N)>,
}

#[async_trait]
impl<
        Params: Agent,
        Returns: Schema,
        S: BroadcastService<Params, Returns>,
        N: Transport + Sync + Send,
    > Runnable for BroadcastApp<Params, Returns, S, N>
{
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick::<String>(None);
        }
    }
}

impl<
        Params: Agent,
        Returns: Schema,
        S: BroadcastService<Params, Returns>,
        N: Transport + Sync + Send,
    > BroadcastApp<Params, Returns, S, N>
{
    pub async fn new(
        service: S,
        transport: Arc<N>,
        health_checker: Healthchecker,
    ) -> Result<Self, BaseError<Value>> {
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
                        log::debug!("got response schema{:#?}", response_schema);
                        Ok(())
                    }
                    Err(error) => {
                        log::debug!("Got response left: {:#?}", error);
                        Err(error)
                    }
                },
                Err(error) => {
                    log::debug!("got error {:#?}", error);
                    Ok(())
                }
            };
            result?;
            message.ok().await?;
        }
    }
}
