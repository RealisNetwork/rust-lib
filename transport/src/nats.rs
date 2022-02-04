use crate::traits::{MessageReceiver, Transport};
use async_trait::async_trait;
use error_registry::{Nats as NatsError, RealisErrors};
use futures::StreamExt;
use ratsio::{StanClient, StanMessage, StanOptions, StanSid};
use std::sync::Arc;

#[derive(Clone)]
pub struct Nats {
    stan_client: Arc<StanClient>,
}

impl Nats {
    pub async fn new(nats_opts: &str, client_id: &str, cluster_id: &str) -> Self {
        let opts = StanOptions::with_options(nats_opts, cluster_id, &client_id[..]);
        let stan_client = StanClient::from_options(opts).await.unwrap(); // TODO handle this unwrap
        Self { stan_client }
    }
}

#[async_trait]
impl Transport for Nats {
    type Error = RealisErrors;
    type Message = Vec<u8>;
    type MessageId = StanMessage;
    type SubscribeId = StanSid;

    async fn publish(
        &self,
        topic: &str,
        message: Self::Message,
        _topic_res: Option<String>,
    ) -> Result<(), Self::Error> {
        self.stan_client
            .publish(topic, &message)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Send))
    }

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error> {
        let (stan_id, mut stream) = self
            .stan_client
            .subscribe(topic, None, None)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Disconnected))?;

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
            .map_err(|_| RealisErrors::Nats(NatsError::Unsubscribe))
    }

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error> {
        self.stan_client
            .acknowledge(message_id)
            .await
            .map_err(|_| RealisErrors::Nats(NatsError::Ok))
    }
}
