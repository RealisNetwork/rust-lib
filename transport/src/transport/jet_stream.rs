use std::time::Duration;

use async_trait::async_trait;
use jet_stream::jetstream::{self, JetStream, StreamConfig, SubscribeOptions};
use serde_json::Value;

use error_registry::{BaseError, ErrorType};
use error_registry::custom_errors::{CustomErrorType, Nats as CustomNats};
use error_registry::generated_errors::{GeneratedError, Nats as GeneratedNats};

use crate::{Transport, VReceivedMessage, VResponse, VSubscription};
use crate::common::TransportResult;
use crate::subscription::Subscription;


pub struct JetTransport {
    stream: JetStream,

}

impl JetTransport {
    pub fn new(nats_url: &str) -> TransportResult<JetTransport> {
        let nats = jet_stream::connect(nats_url)
            .map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected)),
                    None,
                )
            })?;
        let jet_stream = jetstream::new(nats);


        Ok(Self {
            stream: jet_stream,
        }
        )
    }
}

#[async_trait]
impl Transport for JetTransport {
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
            self.stream.publish(&topic_res, response).map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    CustomErrorType::Nats(CustomNats::Send).into(),
                    None,
                )
            })?;
            Ok(())
        })
    }


    /// TODO:durable name?
    async fn subscribe(&self, topic: &str) -> TransportResult<VSubscription> {
        let durable_name = format!("{}_{}", topic, "some_name");
        let subscription_options = SubscribeOptions::new()
            .deliver_last()
            .durable_name(durable_name);

        self
            .stream
            .add_stream(topic);

        self
            .stream
            .subscribe_with_options(topic, &subscription_options)
            .map(|subscription| subscription.into())
            .map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    GeneratedError::Nats(GeneratedNats::InternalServiceCall).into(),
                    None,
                )
            })
    }

    async fn subscribe_not_durable(&self, topic: &str) -> TransportResult<VSubscription> {
        let subscription_options = SubscribeOptions::new()
            .deliver_last();
        self
            .stream
            .add_stream(topic);

        self
            .stream
            .subscribe_with_options(topic, &subscription_options)
            .map(|subscription| subscription.into())
            .map_err(|error| {
                BaseError::<Value>::new(
                    format!("{:?}", error),
                    GeneratedError::Nats(GeneratedNats::InternalServiceCall).into(),
                    None,
                )
            })
    }

    async fn send_message_and_observe_reply(&self, topic_response: String, msg: VResponse, max_duration: Option<Duration>) -> TransportResult<VReceivedMessage> {
        let mut subscription = self.subscribe_not_durable(topic_response.as_str()).await?;

        self.publish(msg).await?;

        let message_result = subscription
            .next_timeout(max_duration.unwrap_or_else(|| Duration::from_secs(25)))
            .await;

        subscription.unsubscribe().await?;
        self
            .stream
            .purge_stream(topic_response);

        Ok(message_result?)
    }
}