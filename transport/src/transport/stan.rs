use crate::common::TransportResult;
use crate::message::ReceivedMessage;
use crate::response::VResponse;
use crate::subscription::{Subscription, VSubscription};
use crate::{Transport, VReceivedMessage};
use async_trait::async_trait;
use error_registry::custom_errors::Nats as CustomNats;
use error_registry::generated_errors::Nats as GeneratedNats;
use error_registry::BaseError;
use healthchecker::Alivable;
use serde_json::Value;
use stan::{Client, SubscriptionConfig, SubscriptionStart};
use std::time::Duration;

#[derive(Clone)]
pub struct StanTransport {
    pub client_id: String,
    pub client: Client,
}

impl StanTransport {
    pub fn new(url: &str, cluster_id: &str, client_id: &str) -> TransportResult<Self> {
        let nats = nats::connect(url)
            .map_err(|error| BaseError::new(format!("{:?}", error), CustomNats::Disconnected))?;
        let stan = stan::connect(nats, cluster_id, client_id)
            .map_err(|error| BaseError::new(format!("{:?}", error), CustomNats::Disconnected))?;

        Ok(Self {
            client_id: client_id.to_owned(),
            client: stan,
        })
    }
}

#[async_trait]
impl Alivable for StanTransport {
    #[allow(unused_must_use)]
    async fn is_alive(&self) -> bool {
        // TODO: fixme, try use method like ping for this, current solution works, but it is unappropriated
        let res = self.subscribe_not_durable("Healthchecker_ping").await;
        if let Ok(subscription) = res {
            subscription.unsubscribe().await.is_ok()
        } else {
            false
        }
    }

    async fn info(&self) -> &'static str {
        "StanTransport"
    }
}

#[async_trait]
impl Transport for StanTransport {
    async fn publish(&self, response: VResponse) -> TransportResult<()> {
        let (topic_res, response) = match response {
            VResponse::Response(response) => (response.topic_res, response.response),
        };

        log::debug!(
            "Publishing: {:#?} by topic: {}",
            serde_json::from_slice::<Value>(&response),
            topic_res
        );

        tokio::task::block_in_place(move || {
            self.client
                .publish(&topic_res, response)
                .map_err(|error| BaseError::new(format!("{:?}", error), CustomNats::Send).into())
        })
    }

    async fn subscribe(&self, topic: &str) -> TransportResult<VSubscription> {
        let durable_name = format!("{}_{}", topic, self.client_id);
        let subscription_config = SubscriptionConfig {
            queue_group: None,
            durable_name: Some(&durable_name),
            start: SubscriptionStart::LastReceived,
            ..Default::default()
        };
        tokio::task::block_in_place(move || {
            self.client
                .subscribe(topic, subscription_config)
                .map(|subscription| subscription.into())
                .map_err(|error| {
                    BaseError::new(format!("{:?}", error), GeneratedNats::InternalServiceCall)
                        .into()
                })
        })
    }

    async fn subscribe_not_durable(&self, topic: &str) -> TransportResult<VSubscription> {
        let subscription_config = SubscriptionConfig {
            queue_group: None,
            durable_name: None,
            start: SubscriptionStart::LastReceived,
            ..Default::default()
        };
        tokio::task::block_in_place(move || {
            self.client
                .subscribe(topic, subscription_config)
                .map(|subscription| subscription.into())
                .map_err(|error| {
                    BaseError::new(format!("{:?}", error), GeneratedNats::InternalServiceCall)
                        .into()
                })
        })
    }

    async fn send_message_and_observe_reply(
        &self,
        topic_response: String,
        msg: VResponse,
        max_duration: Option<Duration>,
    ) -> TransportResult<VReceivedMessage> {
        let subscription_config = SubscriptionConfig {
            queue_group: None,
            durable_name: None,
            start: SubscriptionStart::NewOnly,
            ..Default::default()
        };
        let mut subscription: VSubscription = tokio::task::block_in_place(move || {
            self.client
                .subscribe(&topic_response, subscription_config)
                .map(|subscription| subscription.into())
                .map_err(|error| {
                    BaseError::new(format!("{:?}", error), GeneratedNats::InternalServiceCall)
                })
        })?;

        self.publish(msg).await?;

        let message_result = subscription
            .next_timeout(max_duration.unwrap_or_else(|| Duration::from_secs(25)))
            .await;

        let message = message_result?;

        message.clone().ok().await?;

        subscription.unsubscribe().await?;

        Ok(message)
    }
}
