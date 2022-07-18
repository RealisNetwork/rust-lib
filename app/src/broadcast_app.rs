use crate::app::{AsyncTryFrom, GetHealthchecker, GetTransport, Runnable};
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
pub struct BroadcastApp<P: Agent, G: Schema, S: BroadcastService<P, G>, N: Transport + Sync + Send>
{
    service: S,
    subscription: VSubscription,
    health_checker: HealthChecker,
    _marker: std::marker::PhantomData<(P, G, N)>,
}

#[async_trait]
impl<T, P, G, ServiceInner, N> AsyncTryFrom<Arc<T>> for BroadcastApp<P, G, ServiceInner, N>
where
    T: 'static + Clone + Send + Sync + GetTransport<N> + GetHealthchecker,
    P: Agent,
    G: Schema,
    ServiceInner: 'static + From<Arc<T>> + BroadcastService<P, G>,
    N: 'static + Transport + Sync + Send,
{
    type Error = BaseError<Value>;

    async fn async_try_from(dependency_container: Arc<T>) -> Result<Self, BaseError<Value>> {
        BroadcastApp::new(
            ServiceInner::from(dependency_container.clone()),
            dependency_container.get_transport(),
            dependency_container.get_healthchecker(),
        )
        .await
    }
}

#[async_trait]
impl<P: Agent, G: Schema, S: BroadcastService<P, G>, N: Transport + Sync + Send> Runnable
    for BroadcastApp<P, G, S, N>
{
    async fn run(&mut self) {
        let health_checker = self.health_checker.clone();
        if let Err(error) = self.run_internal().await {
            log::error!("{:?}", error);
            health_checker.make_sick::<String>(None);
        }
    }
}

impl<P: Agent, G: Schema, S: BroadcastService<P, G>, N: Transport + Sync + Send>
    BroadcastApp<P, G, S, N>
{
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
