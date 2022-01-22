use crate::traits::{MessageReceiver, Transport};
use async_trait::async_trait;
use ratsio::{StanClient, StanOptions, StanSid};
use rust_lib::error_registry::{Nats as NatsError, RealisErrors};
use std::sync::Arc;
use futures::StreamExt;

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
    type Message = Vec<u8>;
    type Error = RealisErrors;
    type SubscribeId = StanSid;

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        self.stan_client.publish(topic, &message).await
            .map_err(|_| RealisErrors::Nats(NatsError::Send))
    }

    async fn subscribe<'a>(&self, topic: &str, callback: impl MessageReceiver<Self::Message, Self::Error> + 'a) -> Result<(), Self::Error> {
        let (stan_id, stream) = self.stan_client.subscribe(topic, None, None).await
            .map_err(|_| RealisErrors::Nats(NatsError::Disconnected))?;

        let mut stream = stream.map(|stan_message| stan_message.payload);

        loop {
            match stream.next().await {
                None => {
                    self.unsubscribe(stan_id).await?;
                    break;
                }
                Some(message) => {
                    if let Err(error) = callback.process(message.clone()).await {
                        self.unsubscribe(stan_id).await?;
                        self.publish(topic, message, None).await?;
                        return Err(error)
                    }
                }
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
}