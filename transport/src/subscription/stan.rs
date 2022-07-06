use crate::common::TransportResult;
use crate::{Subscription, VReceivedMessage};
use ::stan::Subscription as LibStanSubscription;
use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
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
                .ok_or(BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))
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
                .map_err(|err| {
                    BaseError::<Value>::new(
                        err.to_string(),
                        CustomErrorType::Nats(CustomNats::Timeout).into(),
                        None,
                    )
                })
        })
    }
    async fn unsubscribe(mut self) -> TransportResult<()> {
        tokio::spawn(async move {
            self.subscription.unsubscribe().map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    CustomErrorType::Nats(CustomNats::Unsubscribe).into(),
                    None,
                )
            })
        })
        .await
        .map_err(|err| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))?
    }
}
