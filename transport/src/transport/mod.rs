pub mod stan;

use crate::response::VResponse;
use crate::subscription::VSubscription;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use crate::common::TransportResult;
use crate::transport::stan::StanTransport;

#[async_trait]
#[enum_dispatch]
pub trait Transport {
    async fn publish(&mut self, response: VResponse) -> TransportResult<()>;

    async fn subscribe(&mut self, topic: &str) -> TransportResult<VSubscription>;
}

#[enum_dispatch(Transport)]
pub enum VTransport {
    Stan(StanTransport),
}