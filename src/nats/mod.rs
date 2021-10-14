pub mod response_builder;

use crate::primitives::adapter::Error;
use ratsio::{self, StanClient, StanOptions};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};

#[must_use]
struct NatsStream {
    connection: Arc<StanClient>,
}

impl NatsStream {
    #[must_use]
    pub async fn new(
        subject: &str,
        client_id: &str,
        cluster_id: &str,
        address: &str,
    ) -> Result<Self, Error> {
        let client_id = format!("{}-{}", client_id, subject);
        let opts = StanOptions::with_options(address, cluster_id, &client_id[..]);

        let stan_client = StanClient::from_options(opts)
            .await
            .map_err(Error::NatsError)?;

        Ok(Self {
            connection: stan_client,
        })
    }

    #[must_use]
    pub async fn subscribe(&self, sender: Sender<Value>) {}

    // TODO rename
    #[must_use]
    pub async fn responder(&self, receiver: Receiver<Value>) {}
}
