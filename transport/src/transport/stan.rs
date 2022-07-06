use crate::common::TransportResult;
use crate::message::ReceivedMessage;
use crate::response::VResponse;
use crate::subscription::{Subscription, VSubscription};
use crate::{Response, Transport};
use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{GeneratedError, Nats as GeneratedNats};
use error_registry::BaseError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stan::{Client, SubscriptionConfig, SubscriptionStart};

pub struct StanTransport {
    pub client_id: String,
    pub client: Client,
}

impl StanTransport {
    pub fn new(url: &str, cluster_id: &str, client_id: &str) -> TransportResult<Self> {
        let nats = nats::connect(url).map_err(|error| {
            BaseError::new(
                format!("{:?}", error),
                CustomErrorType::Nats(CustomNats::Disconnected).into(),
                None,
            )
        })?;
        let stan = stan::connect(nats, cluster_id, client_id).map_err(|error| {
            BaseError::new(
                format!("{:?}", error),
                CustomErrorType::Nats(CustomNats::Disconnected).into(),
                None,
            )
        })?;

        Ok(Self {
            client_id: client_id.to_owned(),
            client: stan,
        })
    }
}

#[async_trait]
impl Transport for StanTransport {
    async fn publish(&self, response: VResponse) -> TransportResult<()> {
        let (topic_res, response) = match response {
            VResponse::Response(response) => (response.topic_res, response.response),
        };

        tokio::task::block_in_place(move || {
            self.client.publish(&topic_res, response).map_err(|error| {
                BaseError::new(
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
                    BaseError::new(
                        format!("{:?}", error),
                        GeneratedError::Nats(GeneratedNats::InternalServiceCall).into(),
                        None,
                    )
                })
        })
    }

    async fn send_message_and_observe_reply<
        Schema: DeserializeOwned + Send,
        SendSchema: Serialize + Send + Sync,
    >(
        &self,
        topic_response: String,
        publish_topic: String,
        msg: SendSchema,
        max_duration: Option<u64>,
    ) -> TransportResult<Schema> {
        let mut subscription = self.subscribe(topic_response.as_str()).await?;

        self.publish(VResponse::Response(Response::new(
            publish_topic.as_str(),
            serde_json::to_vec(&msg)
                .map_err(|err| BaseError::from(CustomErrorType::Nats(CustomNats::CantSerialize)))?,
        )))
        .await?;

        let message = subscription
            .next_timeout(std::time::Duration::from_secs(max_duration.unwrap_or(25)))
            .await?;

        let result = message
            .deserialize::<Schema>()
            .map_err(|_| BaseError::from(GeneratedError::Nats(GeneratedNats::Receive)));

        message.ok();

        subscription.unsubscribe().await;

        result
    }
}
