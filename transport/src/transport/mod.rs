pub mod jet_stream;
pub mod stan;

use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::VSubscription;
use crate::transport::jet_stream::JetTransport;
use crate::transport::stan::StanTransport;
use crate::Response;
use crate::VReceivedMessage;
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use error_registry::generated_errors::{Common, GeneratedError};
use error_registry::BaseError;
use serde::Serialize;
use healthchecker::Alivable;
use std::time::Duration;

#[async_trait]
#[enum_dispatch]
pub trait Transport {
    async fn publish(&self, response: VResponse) -> TransportResult<()>;

    async fn raw_publish<M: Serialize + Sync>(
        &self,
        topic: String,
        message: &M,
    ) -> TransportResult<()> {
        let payload = serde_json::to_vec(&message).map_err(|error| {
            BaseError::<()>::new(
                format!("{:?}", error),
                GeneratedError::Common(Common::InternalServerError).into(),
                None,
            )
        })?;

        self.publish(VResponse::Response(Response {
            topic_res: topic,
            response: payload,
        }))
        .await
    }

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
#[enum_dispatch(Alivable)]
pub enum VTransport {
    Stan(StanTransport),
    Jet(JetTransport),
}

#[async_trait]
impl Alivable for VTransport {
    async fn is_alive(&self) -> bool {
        match self {
            VTransport::Stan(stan) => stan.is_alive().await,
            VTransport::Jet(jet) => jet.is_alive().await,
        }
    }

    async fn info(&self) -> &'static str {
        match self {
            VTransport::Stan(stan) => stan.info().await,
            VTransport::Jet(jet) => jet.info().await,
        }
    }
}
