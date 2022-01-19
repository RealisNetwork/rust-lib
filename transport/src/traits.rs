use async_trait::async_trait;
use tokio::time::{error::Elapsed, timeout, Duration};
use futures::{Stream, StreamExt, Future};

#[async_trait]
pub trait Transport {
    type Message: Send;
    type Error: From<Elapsed>;
    type SubscribeId;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error>;

    async fn subscribe(&self, topic: String) -> Result<(Self::SubscribeId, Box<dyn Future<Output=dyn Stream<Item = Self::Message>>>), Self::Error>;

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error>;

    async fn get_one(&self, topic: String) -> Result<Option<Self::Message>, Self::Error> {
        let (subscribe_id, stream) = self.subscribe(topic).await?;
        let message = stream.next().await;
        self.unsubscribe(subscribe_id).await?;
        Ok(message)
    }

    async fn message_reply(
        &self,
        topic: String,
        topic_res: String,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Option<Self::Message>, Self::Error> {
        self.publish(&topic, message, Some(topic_res.to_string())).await?;
        match duration {
            Some(duration) => timeout(duration, self.get_one(topic_res)).await?,
            None => timeout(Duration::from_secs(25), self.get_one(topic_res)).await?,
        }
    }
}
