use std::fmt::Debug;

use async_trait::async_trait;
use backtrace::Backtrace;
use error_registry::custom_errors::Nats as CustomNats;
use error_registry::generated_errors::Common;
use error_registry::{BaseError, ErrorType};
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
            let msg = format!("{:?}", error);
            let error_type: ErrorType = error.into();
            let trace = Backtrace::new();
            BaseError {
                msg,
                error_type: Common::InternalServerError.into(),
                data: serde_json::from_slice(&self.message.data).ok(),
                trace: format!("{:?}", trace),
                status: error_type.into(),
            }
        });
        log::debug!("Deserialized: {:#?}", &deserialized);
        deserialized
    }

    async fn ok(self) -> TransportResult<()> {
        log::debug!("Okaying: {:#?}", &self);
        tokio::task::block_in_place(move || {
            self.message.ack().map_err(|error| {
                let msg = format!("{:?}", error);
                let error_type: ErrorType = error.into();
                let trace = Backtrace::new();
                BaseError {
                    msg,
                    error_type: CustomNats::Ok.into(),
                    data: serde_json::to_value(self.message.data).ok(),
                    trace: format!("{:?}", trace),
                    status: error_type.into(),
                }
            })
        })
    }
}
