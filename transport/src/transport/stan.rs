use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::{Subscription, VSubscription};
use crate::{Transport, VReceivedMessage};
use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{GeneratedError, Nats as GeneratedNats};
use error_registry::{BaseError, ErrorType};
use healthchecker::{Alivable, HealthChecker};
use nats::Connection;
use serde_json::Value;
use stan::{Client, SubscriptionConfig, SubscriptionStart};
use std::time::Duration;
use uuid::Uuid;

pub struct StanTransport {
    pub client_id: String,
    pub client: Client,
    connection: Connection,
}

impl StanTransport {
    pub fn new(url: &str, cluster_id: &str, client_id: &str) -> TransportResult<Self> {
        let nats = nats::connect(url).map_err(|error| {
            BaseError::<Value>::new(
                format!("{:?}", error),
                ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected)),
                None,
            )
        })?;
        let stan = stan::connect(nats.clone(), cluster_id, client_id).map_err(|error| {
            BaseError::<Value>::new(
                format!("{:?}", error),
                CustomErrorType::Nats(CustomNats::Disconnected).into(),
                None,
            )
        })?;

        Ok(Self {
            client_id: client_id.to_owned(),
            client: stan,
            connection: nats,
        })
    }
}

#[async_trait]
impl Alivable for StanTransport {
    async fn is_alive(&self) -> bool {
        self.connection.rtt().is_ok()
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
            self.client.publish(&topic_res, response).map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    CustomErrorType::Nats(CustomNats::Send).into(),
                    None,
                )
            })
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
                    BaseError::<Value>::new(
                        format!("{:?}", error),
                        GeneratedError::Nats(GeneratedNats::InternalServiceCall).into(),
                        None,
                    )
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
                    BaseError::<Value>::new(
                        format!("{:?}", error),
                        GeneratedError::Nats(GeneratedNats::InternalServiceCall).into(),
                        None,
                    )
                })
        })
    }

    async fn send_message_and_observe_reply(
        &self,
        topic_response: String,
        msg: VResponse,
        max_duration: Option<Duration>,
    ) -> TransportResult<VReceivedMessage> {
        let mut subscription = self.subscribe_not_durable(topic_response.as_str()).await?;

        self.publish(msg).await?;

        let message_result = subscription
            .next_timeout(max_duration.unwrap_or_else(|| Duration::from_secs(25)))
            .await;

        subscription.unsubscribe().await?;

        Ok(message_result?)
    }
}
