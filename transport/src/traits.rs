// use async_trait::async_trait;
// use tokio::{
//     sync::{
//         oneshot::{channel, error::RecvError, Sender},
//         Mutex,
//     },
//     time::{error::Elapsed, timeout, Duration},
// };
//
// #[async_trait]
// pub trait MessageReceiver<M, O, E>: Send + Sync {
//     /// # Returns
//     ///
//     /// * true - to continue process messages
//     /// * false - to stop receive new messages
//     async fn process(&self, message: M, message_id: O) -> Result<bool, E>;
// }
//
// #[async_trait]
// pub trait Transport {
//     type Message: Send;
//     type Error: From<Elapsed> + From<RecvError> + From<Self::Message> + Send;
//     type SubscribeId;
//     type MessageId: Send;
//
//     async fn publish(&self, topic: &str, message: Self::Message, topic_res:
// Option<String>) -> Result<(), Self::Error>;
//
//     async fn subscribe<'b>(
//         &self,
//         topic: &str,
//         callback: impl MessageReceiver<Self::Message, Self::MessageId,
// Self::Error> + 'b,     ) -> Result<(), Self::Error>;
//
//     async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) ->
// Result<(), Self::Error>;
//
//     async fn observe_reply(&self, topic: &str) -> Result<Self::Message,
// Self::Error> {         let (tx, rx) = channel();
//
//         let receiver = ObserveReplyReceiver {
//             tx: Mutex::new(Some(tx)),
//         };
//         self.subscribe(topic, receiver).await?;
//
//         let (message, message_id) = rx.await?;
//         self.ok(message_id).await?;
//
//         Ok(message)
//     }
//
//     async fn message_reply(
//         &self,
//         topic: &str,
//         topic_res: &str,
//         message: Self::Message,
//         duration: Option<Duration>,
//     ) -> Result<Self::Message, Self::Error> {
//         let (tx, rx) = channel();
//
//         let receiver = ObserveReplyReceiver {
//             tx: Mutex::new(Some(tx)),
//         };
//
//         tokio::spawn({
//             async move { self.subscribe(topic_res, receiver) }
//         });
//
//         self.publish(&topic, message, Some(topic_res.to_string())).await?;
//
//         match duration {
//             Some(duration) => timeout(duration,
// self.observe_reply(topic_res)).await?,             None =>
// timeout(Duration::from_secs(25), self.helper(rx)).await?,         }
//     }
//
//     async fn ok(&self, message_id: Self::MessageId) -> Result<(),
// Self::Error>;
//
//     async fn helper(&self, rx: tokio::sync::oneshot::Receiver<(Self::Message,
// Self::MessageId)>) -> Result<Self::Message, Self::Error> {         let
// (message, message_id) = rx.await?;         self.ok(message_id).await?;
//
//         Ok(message)
//     }
// }
//
// struct ObserveReplyReceiver<M, O> {
//     tx: Mutex<Option<Sender<(M, O)>>>,
// }
//
// #[async_trait]
// impl<M: Send, O: Send, E: From<M> + Send> MessageReceiver<M, O, E> for
// ObserveReplyReceiver<M, O> {     async fn process(&self, message: M,
// message_id: O) -> Result<bool, E> {         match self.tx.lock().await.take()
// {             None => Err(message)?,
//             Some(tx) => tx.send((message, message_id)).map_err(|(message, _)|
// message)?,         }
//         Ok(false)
//     }
// }

use async_trait::async_trait;
use tokio::{
    sync::oneshot::error::RecvError,
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
