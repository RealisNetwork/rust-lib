use async_trait::async_trait;
use tokio::{
    sync::{
        oneshot::error::RecvError,
    },
    time::{error::Elapsed, Duration},
};

#[async_trait]
pub trait MessageReceiver<M, O, E>: Send + Sync {
    /// # Returns
    ///
    /// * true - to continue process messages
    /// * false - to stop receive new messages
    async fn process(&self, message: M, message_id: O) -> Result<bool, E>;
}

#[async_trait]
pub trait Transport {
    type Message: Send;
    type Error: From<Elapsed> + From<RecvError> + From<Self::Message> + Send;
    type SubscribeId;
    type MessageId: Send;

    async fn publish(&self, topic: &str, message: Self::Message, topic_res: Option<String>) -> Result<(), Self::Error>;

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error>;

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error>;

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error>;

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error>;
}
