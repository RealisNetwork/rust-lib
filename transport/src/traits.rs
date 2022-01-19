use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::{error::Elapsed, timeout, Duration};
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::{channel, Sender};

#[async_trait]
pub trait MessageReceiver<M, E>: Send + Sync {
    async fn process(&self, message: M) -> Result<(), E>;
}

#[async_trait]
pub trait Transport {
    type Message: Send;
    type Error: From<Elapsed> + From<TryRecvError> + From<Self::Message> + Send;
    type SubscribeId;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error>;

    async fn subscribe<'a>(&self, topic: &str, callback: impl MessageReceiver<Self::Message, Self::Error> + 'a) -> Result<(), Self::Error>;

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error>;

    async fn observe_reply(&self, topic: &str) -> Result<Self::Message, Self::Error> {
        let (tx, mut rx) = channel();

        let receiver = ObserveReplyReceiver { tx: Mutex::new(Some(tx)) };
        self.subscribe(topic, receiver).await?;

        Ok(rx.try_recv()?)
    }

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error> {
        self.publish(&topic, message, Some(topic_res.to_string())).await?;
        match duration {
            Some(duration) => timeout(duration, self.observe_reply(topic_res)).await?,
            None => timeout(Duration::from_secs(25), self.observe_reply(topic_res)).await?,
        }
    }
}

struct ObserveReplyReceiver<M> {
    tx: Mutex<Option<Sender<M>>>,
}

#[async_trait]
impl<M: Send, E: From<M> + Send> MessageReceiver<M, E> for ObserveReplyReceiver<M> {
    async fn process(&self, message: M) -> Result<(), E> {
        match self.tx.lock()
            .await
            .take() {
            None => Err(message)?,
            Some(tx) => tx.send(message)?,
        }
        Ok(())
    }
}
