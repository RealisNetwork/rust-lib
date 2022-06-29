use async_trait::async_trait;
use serde::de::DeserializeOwned;
use stan::Message;
use error_registry::BaseError;
use error_registry::generated_errors::{Common, GeneratedError};
use crate::common::TransportResult;
use crate::ReceivedMessage;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};

#[derive(Debug)]
pub struct StanMessage {
    pub message: Message,
}

#[async_trait]
impl ReceivedMessage for StanMessage {
    fn deserialize<T: DeserializeOwned>(&self) -> TransportResult<T> {
        serde_json::from_slice(&self.message.data).map_err(|error| {
            BaseError::new(
                format!("{:?}", error),
                GeneratedError::Common(Common::InternalServerError).into(),
                serde_json::to_value(&self.message.data).ok(),
            )
        })
    }

    async fn ok(self) -> TransportResult<()> {
        tokio::task::block_in_place(move || {
            self.message.ack().map_err(|error| {
                BaseError::new(
                    format!("{:?}", error),
                    CustomErrorType::Nats(CustomNats::Ok).into(),
                    serde_json::to_value(self.message.data).ok(),
                )
            })
        })
    }
}