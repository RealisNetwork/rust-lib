pub mod jet;
pub mod stan;

use crate::common::TransportResult;
use crate::message::jet::JetMessage;
use crate::message::stan::StanMessage;
use ::stan::Message;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

#[async_trait]
#[enum_dispatch]
pub trait ReceivedMessage {
    fn deserialize<T: DeserializeOwned + Debug>(&self) -> TransportResult<T>;

    async fn ok(self) -> TransportResult<()>;
}

#[enum_dispatch(ReceivedMessage)]
#[derive(Debug, Clone)]
pub enum VReceivedMessage {
    Stan(StanMessage),
    Jet(JetMessage),
}

impl From<Message> for VReceivedMessage {
    fn from(message: Message) -> Self {
        VReceivedMessage::Stan(StanMessage { message })
    }
}

impl From<jet_stream::Message> for VReceivedMessage {
    fn from(message: jet_stream::Message) -> Self {
        VReceivedMessage::Jet(JetMessage { message })
    }
}
