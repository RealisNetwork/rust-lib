use crate::traits::{MessageReceiver, Transport};
use async_trait::async_trait;
use error_registry::{
    custom_errors::{CustomErrorType, Nats as CustomNatsError},
    generated_errors::{GeneratedError, Nats as NatsError, Nats::Send},
    BaseError, ErrorType,
};
use futures::StreamExt;
use ratsio::{StanClient, StanMessage, StanOptions, StanSid, StartPosition};
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

#[derive(Clone)]
pub struct Nats {
    pub stan_client: Arc<StanClient>,
}

impl Nats {
    pub async fn new(nats_opts: &str, client_id: &str, cluster_id: &str) -> Result<Self, ratsio::RatsioError> {
        let opts = StanOptions::with_options(nats_opts, cluster_id, &client_id[..]);
        let stan_client = match StanClient::from_options(opts).await {
            Ok(stan_client_) => stan_client_,
            Err(err) => return Err(err),
        };
        Ok(Self { stan_client })
    }
}

#[async_trait]
impl Transport for Nats {
    type Error = BaseError<()>;
    type Message = Vec<u8>;
    type MessageId = StanMessage;
    type SubscribeId = StanSid;

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        self.stan_client.publish(topic, &message).await.map_err(|_| {
            BaseError::new(
                "Can not Send to nats".to_string(),
                ErrorType::Generated(GeneratedError::Nats(NatsError::Send)),
                None,
            )
        })
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
        secs: i32,
    ) -> Result<(), Self::Error> {
        let (stan_id, mut stream) = self
            .stan_client
            .subscribe_with_all(topic, None, None, 1024, secs, StartPosition::LastReceived, 0, None, true)
            .await
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Disconnected)))?;

        loop {
            match stream.next().await {
                None => {
                    self.unsubscribe(stan_id).await?;
                    break;
                }
                Some(message) => match callback.process(message.payload.clone(), message).await {
                    Ok(true) => {}
                    Ok(false) => {
                        self.unsubscribe(stan_id).await?;
                        break;
                    }
                    Err(error) => {
                        self.unsubscribe(stan_id).await?;
                        return Err(error);
                    }
                },
            }
        }

        Ok(())
    }

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error> {
        self.stan_client
            .un_subscribe(&subscribe_id)
            .await
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)))
    }

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error> {
        let (stan_id, mut stream) = self
            .stan_client
            .subscribe(topic_res, None, None)
            .await
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Disconnected)))?;

        self.publish(topic, message, None).await?;

        let option_message = match duration {
            Some(duration) => timeout(duration, stream.next()).await?,
            None => timeout(Duration::from_secs(25), stream.next()).await?,
        };

        self.unsubscribe(stan_id).await?;

        let message = option_message.ok_or(BaseError::from(CustomErrorType::Nats(CustomNatsError::Receive)))?;

        self.ok(message.clone()).await?;

        Ok(message.payload)
    }

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error> {
        self.stan_client
            .acknowledge(message_id)
            .await
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Ok)))
    }
}
