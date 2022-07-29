use std::fmt::Debug;

use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use jet_stream::Message;
use serde::de::DeserializeOwned;

use crate::common::TransportResult;
use crate::ReceivedMessage;

#[derive(Debug)]
pub struct JetMessage {
    pub message: Message,
}

#[async_trait]
impl ReceivedMessage for JetMessage {
    fn deserialize<T: DeserializeOwned + Debug>(&self) -> TransportResult<T> {
        log::debug!("Deserializing: {:#?}", &self);
        let deserialized = serde_json::from_slice(&self.message.data).map_err(|error| {
            BaseError::new(
                format!("{:?}", error),
                GeneratedError::Common(Common::InternalServerError).into(),
                serde_json::from_slice(&self.message.data).ok(),
            )
        });
        log::debug!("Deserialized: {:#?}", &deserialized);
        deserialized
    }

    async fn ok(self) -> TransportResult<()> {
        log::debug!("Okaying: {:#?}", &self);
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
