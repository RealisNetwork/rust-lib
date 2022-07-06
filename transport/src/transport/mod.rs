pub mod stan;

use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::VSubscription;
use crate::transport::stan::StanTransport;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
#[enum_dispatch]
pub trait Transport {
    async fn publish(&self, response: VResponse) -> TransportResult<()>;

    async fn subscribe(&self, topic: &str) -> TransportResult<VSubscription>;

    async fn send_message_and_observe_reply<
        Schema: DeserializeOwned + Send,
        SendSchema: Serialize + Send + Sync,
    >(
        &mut self,
        topic_response: String,
        publish_topic: String,
        msg: SendSchema,
        max_duration: Option<u64>,
    ) -> TransportResult<Schema>;
}

#[enum_dispatch(Transport)]
pub enum VTransport {
    Stan(StanTransport),
}
