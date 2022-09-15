use crate::common::TransportResult;
use crate::{Subscription, VReceivedMessage};
use ::stan::Subscription as LibStanSubscription;
use async_trait::async_trait;
use error_registry::custom_errors::Nats as CustomNats;
use error_registry::generated_errors::{GeneratedError, Nats};
use error_registry::BaseError;
use serde_json::Value;

pub struct StanSubscription {
    pub subscription: LibStanSubscription,
}

#[async_trait]
impl Subscription for StanSubscription {
    async fn next(&mut self) -> TransportResult<VReceivedMessage> {
        tokio::task::block_in_place(move || {
            self.subscription
                .next()
                .map(|message| message.into())
                .ok_or_else(|| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))
        })
    }

    async fn next_timeout(
        &mut self,
        timeout: std::time::Duration,
    ) -> TransportResult<VReceivedMessage> {
        tokio::task::block_in_place(move || {
            self.subscription
                .next_timeout(timeout)
                .map(|message| message.into())
                .map_err(|err| BaseError::new(format!("{:?}", err), CustomNats::Timeout).into())
        })
    }
    async fn unsubscribe(mut self) -> TransportResult<()> {
        tokio::spawn(async move {
            self.subscription.unsubscribe().map_err(|error| {
                BaseError::new(format!("{:?}", error), CustomNats::Unsubscribe).into()
            })
        })
        .await
        .map_err(|_| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))?
    }
}
