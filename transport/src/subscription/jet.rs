use std::time::Duration;

use jet_stream::jetstream::PushSubscription;
use serde_json::Value;
use async_trait::async_trait;

use error_registry::BaseError;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{GeneratedError, Nats};

use crate::{Subscription, VReceivedMessage};
use crate::common::TransportResult;

pub struct JetSubscription {
    pub subscription: PushSubscription,
}

#[async_trait]
impl Subscription for JetSubscription {
    async fn next(&mut self) -> TransportResult<VReceivedMessage> {
        tokio::task::block_in_place(move || {
            self.subscription
                .next()
                .map(|message| message.into())
                .ok_or_else(|| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))
        })
    }


    async fn next_timeout(&mut self, timeout: Duration) -> TransportResult<VReceivedMessage> {
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

    async fn unsubscribe(self) -> TransportResult<()> {
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
            .map_err(|_| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))?
    }
}