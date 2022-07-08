pub mod stan;

use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::VSubscription;
use crate::transport::stan::StanTransport;
use crate::VReceivedMessage;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::time::Duration;

#[async_trait]
#[enum_dispatch]
pub trait Transport {
    async fn publish(&self, response: VResponse) -> TransportResult<()>;

    async fn subscribe(&self, topic: &str) -> TransportResult<VSubscription>;

    async fn subscribe_not_durable(&self, topic: &str) -> TransportResult<VSubscription>;

    async fn send_message_and_observe_reply(
        &self,
        topic_response: String,
        msg: VResponse, //SendSchema,
        max_duration: Option<Duration>,
    ) -> TransportResult<VReceivedMessage>; //Schema>;
}

#[enum_dispatch(Transport)]
pub enum VTransport {
    Stan(StanTransport),
}
