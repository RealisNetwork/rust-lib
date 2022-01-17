use crate::traits::Transport;
use async_trait::async_trait;
use ratsio::{RatsioError, StanClient, StanMessage, StanOptions, StanSid};
use rust_lib::error_registry::{Nats as NatsError, RealisErrors};
use std::{future::Future, io::Read, sync::Arc};
use tokio::{
    select,
    sync::mpsc::{channel, Receiver},
    time::error::Elapsed,
};

#[derive(Clone)]
pub struct Nats {
    stan_client: Arc<StanClient>,
}

impl Nats {
    pub async fn new(nats_opts: &str, client_id: &str, cluster_id: &str) -> Self {
        let opts = StanOptions::with_options(nats_opts, cluster_id, &client_id[..]);
        let stan_client = StanClient::from_options(opts).await.unwrap();
        Self { stan_client }
    }
}

#[async_trait]
impl Transport for Nats {
    type Error = RealisErrors;
    type Message = Vec<u8>;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error> {
        match self.stan_client.publish(topic, &message).await {
            Ok(()) => Ok(()),
            Err(error) => Err(RealisErrors::Nats(NatsError::MessageReplyTimeout)),
        }
    }

    async fn subscribe(&self, topic: &str) -> Result<Receiver<Self::Message>, Self::Error> {
        let (tx, rx) = channel(1024);
        tokio::spawn(async move {
            loop {
                let (stan_id, mut stream) = self.stan_client.subscribe(topic, None, None).await?;
                if let Some(raw_message) = stream.next() {
                    tx.send(raw_message).await;
                }
            }
        });
        Ok(rx)
    }

    async fn get_one(&self, topic: &str) -> Result<Self::Message, Self::Error> {
        let (stan_id, mut stream) = self.stan_client.subscribe(topic, None, None).await?;
        let message = stream.next().await;
        self.stan_client
            .un_subscribe(&stan_id)
            .await
            .map_err(RealisErrors::Nats(NatsError::Disconnected))?;
        Ok(message)
    }
}
