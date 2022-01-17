use ratsio::StanClient;
use tokio::sync::mpsc::Receiver;
use tokio::time::error::Elapsed;
use async_trait::async_trait;
use crate::traits::Transport;

#[derive(Debug, Clone)]
pub struct Nats {
    stan_client: Arc<StanClient>
}

impl Nats {
    pub fn new() -> Result<Self, ()> {
        todo!()
    }
}

#[async_trait]
impl Transport for Nats {
    type Message = Vec<u8>;
    type Error: From<Elapsed>;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error> {
        todo!()
    }

    async fn subscribe(&self, topic: &str) -> Result<Receiver<Self::Message>, Self::Error> {
        todo!()
    }

    async fn get_one(&self, topic: &str) -> Result<Self::Message, Self::Error> {
        todo!()
    }

}