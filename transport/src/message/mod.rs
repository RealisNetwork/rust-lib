pub mod stan;

use crate::common::TransportResult;
use crate::message::stan::StanMessage;
use ::stan::Message;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use serde::de::DeserializeOwned;

#[async_trait]
#[enum_dispatch]
pub trait ReceivedMessage {
    fn deserialize<T: DeserializeOwned>(&self) -> TransportResult<T>;

    async fn ok(self) -> TransportResult<()>;
}

#[enum_dispatch(ReceivedMessage)]
#[derive(Debug)]
pub enum VReceivedMessage {
    Stan(StanMessage),
}

impl From<Message> for VReceivedMessage {
    fn from(message: Message) -> Self {
        VReceivedMessage::Stan(StanMessage { message })
    }
}
