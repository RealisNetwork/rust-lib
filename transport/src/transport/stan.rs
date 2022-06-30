use crate::common::TransportResult;
use crate::response::VResponse;
use crate::subscription::VSubscription;
use crate::Transport;
use async_trait::async_trait;
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{GeneratedError, Nats};
use error_registry::BaseError;
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
    async fn publish(&mut self, response: VResponse) -> TransportResult<()> {
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

    async fn subscribe(&mut self, topic: &str) -> TransportResult<VSubscription> {
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
                        GeneratedError::Nats(Nats::InternalServiceCall).into(),
                        None,
                    )
                })
        })
    }
}
