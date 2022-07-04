pub mod stan;

use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::VSubscription;
use crate::transport::stan::StanTransport;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[async_trait]
#[enum_dispatch]
pub trait Transport {
    async fn publish(&self, response: VResponse) -> TransportResult<()>;

    async fn subscribe(&self, topic: &str) -> TransportResult<VSubscription>;
}

#[enum_dispatch(Transport)]
pub enum VTransport {
    Stan(StanTransport),
}
