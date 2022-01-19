use crate::traits::Transport;
use async_trait::async_trait;
use ratsio::{StanClient, StanOptions};
use rust_lib::error_registry::{Nats as NatsError, RealisErrors};
use std::sync::Arc;
use futures::{Stream, StreamExt};

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

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        self.stan_client.publish(topic, &message).await
            .map_err(|_| RealisErrors::Nats(NatsError::Send))
    }

    // TODO add unsubscribe by some way (callback?)
    async fn subscribe(&self, topic: String) -> Result<Box<dyn Stream<Item = Self::Message>>, Self::Error> {
        let (_, stream) = self.stan_client.subscribe(topic, None, None).await
            .map_err(|_| RealisErrors::Nats(NatsError::Disconnected))?;

        Ok(Box::new(stream.map(|message| message.payload)))
    }
}
