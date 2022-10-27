use crate::app::Runnable;
use async_trait::async_trait;
use error_registry::BaseError;
use healthchecker::Healthchecker;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt::Debug;
use std::sync::Arc;
use transport::{ReceivedMessage, Subscription, Transport, VSubscription};

#[async_trait]
pub trait AnyService<
    Params: DeserializeOwned + Serialize + Sync + Send + Debug,
    Returns: Serialize + Sync + Send + Debug,
>: Send + Sync
{
    fn topic_to_subscribe(&self) -> String;

    async fn process(&mut self, request: Params) -> Result<Returns, BaseError<Value>>;
}

pub struct AnyApp<
    Params: DeserializeOwned + Serialize + Sync + Send + Debug,
    Returns: Serialize + Sync + Send + Debug,
    S: AnyService<Params, Returns>,
    N: Transport + Sync + Send,
> {
    service: S,
    subscription: VSubscription,
    health_checker: Healthchecker,
    _marker: std::marker::PhantomData<(Params, Returns, N)>,
}

#[async_trait]
impl<
       Params: DeserializeOwned + Serialize + Sync + Send + Debug,
        Returns: Serialize + Sync + Send + Debug,
        S: AnyService<Params, Returns>,
        N: Transport + Sync + Send,
    > Runnable for AnyApp<Params, Returns, S, N>
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
       Params: DeserializeOwned + Serialize + Sync + Send + Debug,
        Returns: Serialize + Sync + Send + Debug,
        S: AnyService<Params, Returns>,
        N: Transport + Sync + Send,
    > AnyApp<Params, Returns, S, N>
{
    pub async fn new(
        service: S,
        transport: Arc<N>,
        health_checker: Healthchecker,
    ) -> Result<Self, BaseError<Value>> {
        transport
            .subscribe(&service.topic_to_subscribe())
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
                Ok(request) => {
                    log::info!(
                        "Got {}",
                        json!({
                            "topic": self.service.topic_to_subscribe(),
                            "request": request
                        })
                        .to_string()
                    );
                    match self.service.process(request).await {
                        Ok(response_schema) => {
                            log::debug!("got response schema{:#?}", response_schema);
                            Ok(())
                        }
                        Err(error) => {
                            log::debug!("Got response left: {:#?}", error);
                            Err(error)
                        }
                    }
                },
                Err(error) => {
                    log::error!(
                        "Fail to deserialize {}",
                        json!({
                            "topic": self.service.topic_to_subscribe(),
                            "error": json!(error),
                        })
                        .to_string()
                    );
                    Ok(())
                }
            };
            result?;
            message.ok().await?;
        }
    }
}
