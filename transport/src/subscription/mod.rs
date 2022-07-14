pub mod stan;
pub mod jet;

use crate::common::TransportResult;
use crate::message::VReceivedMessage;
use crate::subscription::stan::StanSubscription;
use ::stan::Subscription as LibStanSubscription;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::time::Duration;
use jet_stream::jetstream::PushSubscription;
use crate::subscription::jet::JetSubscription;
use crate::VSubscription::Jet;

#[async_trait]
#[enum_dispatch]
pub trait Subscription {
    async fn next(&mut self) -> TransportResult<VReceivedMessage>;

    async fn next_timeout(&mut self, timeout: Duration) -> TransportResult<VReceivedMessage>;

    async fn unsubscribe(mut self) -> TransportResult<()>;
}

#[enum_dispatch(Subscription)]
pub enum VSubscription {
    Stan(StanSubscription),
    Jet(JetSubscription),
}

impl From<LibStanSubscription> for VSubscription {
    fn from(subscription: LibStanSubscription) -> Self {
        VSubscription::Stan(StanSubscription { subscription })
    }
}

impl From<PushSubscription> for VSubscription {
    fn from(subscription: PushSubscription) -> Self {
        VSubscription::Jet(JetSubscription {subscription })
    }

}
