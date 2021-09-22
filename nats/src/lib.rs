use primitives::Error;

use tokio::sync::mpsc::{Sender, Receiver};
use ratsio::{StanClient, StanOptions};
use serde_json::Value;
use std::sync::Arc;
use ratsio;

struct NatsStream
{
    connection: Arc<StanClient>
}

impl NatsStream {
    pub async fn new(
        subject: &str,
        client_id: &str,
        cluster_id: &str,
        address: &str,
    ) -> Result<Self, Error> {

        let client_id = format!(
            "{}-{}",
            client_id,
            subject
        );
        let opts = StanOptions::with_options(
            address,
            cluster_id,
            &client_id[..],
        );

        let stan_client = StanClient::from_options(opts).await.map_err(Error::NatsError)?;

        Ok(Self {
            connection: stan_client
        })
    }

    pub async fn subscribe(&self, sender: Sender<Value>) {

    }

    // TODO rename
    pub async fn responder(&self, receiver: Receiver<Value>) {

    }
}
