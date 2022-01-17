use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use tokio::time::{timeout, Duration, error::Elapsed};

#[async_trait]
pub trait Transport {
    type Message: Send;
    type Error: From<Elapsed>;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error>;

    async fn subscribe(&self, topic: &str) -> Result<Receiver<Self::Message>, Self::Error>;

    async fn get_one(&self, topic: &str) -> Result<Self::Message, Self::Error>;

    async fn message_reply(&self, topic: &str, topic_res: &str, message: Self::Message, duration: Option<Duration>) -> Result<Self::Message, Self::Error> {
        self.publish(topic, message, Some(topic_res.to_string())).await?;
        match duration {
            Some(duration) => timeout(duration, self.get_one(topic_res)).await?,
            None => timeout(Duration::from_secs(25), self.get_one(topic_res)).await?,
        }
    }
}
