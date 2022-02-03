use async_trait::async_trait;
use tokio::{
    sync::{
        mpsc::{channel, error::RecvError, Sender},
        Mutex,
    },
    time::{error::Elapsed, timeout, Duration},
};

#[async_trait]
pub trait MessageReceiver<M, O, E>: Send + Sync {
    async fn process(&self, message: M, message_id: O) -> Result<(), E>;
}

#[async_trait]
pub trait Transport {
    type Message: Send;
    type Error: From<Elapsed> + From<()> + From<Self::Message> + Send;
    type SubscribeId;
    type MessageId: Send;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error>;

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error>;

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error>;

    async fn observe_reply(&self, topic: &str) -> Result<Self::Message, Self::Error> {
        let (tx, mut rx) = channel(1);

        let receiver = ObserveReplyReceiver {
            tx,
        };
        self.subscribe(topic, receiver).await?;

        match rx.recv().await {
            Some((message, message_id)) => {
                self.ok(message_id).await?;
                Ok(message)
            }
            None => Err(Self::Error::from(()))
        }
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

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error>;
}

struct ObserveReplyReceiver<M, O> {
    tx: Sender<(M, O)>,
}

#[async_trait]
impl<M: Send, O: Send, E: From<M> + From<()> + Send> MessageReceiver<M, O, E> for ObserveReplyReceiver<M, O> {
    async fn process(&self, message: M, message_id: O) -> Result<(), E> {
        self.tx.send((message, message_id)).await.map_err(|error| error.0.0)?;
        Err(E::from(()))
    }
}
