use std::time::Duration;

use async_trait::async_trait;
use jet_stream::jetstream::PushSubscription;
use serde_json::Value;

use error_registry::custom_errors::Nats as CustomNats;
use error_registry::generated_errors::{GeneratedError, Nats};
use error_registry::BaseError;

use crate::common::TransportResult;
use crate::{Subscription, VReceivedMessage};

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
                .map_err(|err| BaseError::new(format!("{:?}", err), CustomNats::Timeout).into())
        })
    }

    async fn unsubscribe(self) -> TransportResult<()> {
        tokio::spawn(async move {
            self.subscription
                .unsubscribe()
                .map_err(|err| BaseError::new(format!("{:?}", err), CustomNats::Unsubscribe).into())
        })
        .await
        .map_err(|_| BaseError::<Value>::from(GeneratedError::Nats(Nats::Receive)))?
    }
}
