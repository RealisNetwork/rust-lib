use std::sync::Arc;
use std::time::Duration;
use async_nats::{Subscriber};
use crate::traits::{MessageReceiver, Transport};
use nats_v2::asynk::{Connection, Subscription};
use nats_v2::asynk::Message;
use ratsio::StanClient;
use tokio::time::timeout;
use async_trait::async_trait;
use error_registry::{Nats as NatsError, RealisErrors};

#[derive(Clone)]
pub struct Nats_v2 {
    pub client: Connection,
}

impl Nats_v2 {

    pub async fn new(nats_opts: &str, client_id: &str, cluster_id: &str) -> Result<Self, ratsio::RatsioError> {
        let client = nats_v2::asynk::Options::new()
            .with_name(client_id)
            .add_root_certificate(cluster_id)
            .connect(format!("tls://{}", nats_opts))
            .await?;
        Ok(Self { client,})
    }
}

#[async_trait]
impl Transport for Nats_v2 {

    type Error = RealisErrors;
    type Message = Vec<u8>;
    type MessageId = Message;
    type SubscribeId = Subscription;

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        self.client
            .publish(topic, &message)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Send))
    }

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error> {
        let default_timeout_in_secs: i32 = 30;
        self.subscribe_with_timeout(topic, callback, default_timeout_in_secs).await
    }

    async fn subscribe_with_timeout<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
        _secs: i32
    ) -> Result<(), Self::Error> {
        let sub = self.client.subscribe(topic).await?;

        loop {
            match sub.next().await {
                None => {
                    sub.unsubscribe().await?;
                    break;
                },
                Some(message) => {
                    match callback.process(message.data.clone(), message).await {
                        Ok(true) => {}
                        Ok(false) => {
                            sub.unsubscribe().await;
                            break;
                        }
                        Err(error) => {
                            sub.unsubscribe().await;
                            return Err(error);
                        },
                    }
                },
            }
        }
        Ok(())
    }

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error> {
        subscribe_id.unsubscribe().await
            .map_err(|_| RealisErrors::Nats(NatsError::Unsubscribe))
    }

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error> {

        let sub = self
            .client
            .subscribe(topic_res)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Disconnected))?;

        self.publish(topic, message, None).await?;

        let option_message = match duration {
            Some(duration) => timeout(duration, sub.next()).await?,
            None => timeout(Duration::from_secs(25), sub.next()).await?,
        };

        self.unsubscribe(sub).await?;

        let message = option_message.ok_or(RealisErrors::Nats(NatsError::Receive))?;

        self.ok(message.clone()).await?;

        Ok(message.data)
    }

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error> {
        nats_v2::Message::from(message_id).ack().map_err(|_| RealisErrors::Nats(NatsError::Unsubscribe))
    }

}
