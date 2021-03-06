use crate::traits::{MessageReceiver, Transport};
use async_trait::async_trait;
use error_registry::{
    custom_errors::{CustomErrorType, Nats as CustomNatsError},
    generated_errors::{GeneratedError, Nats as NatsError},
    BaseError, ErrorType,
};
pub use jet_nats;
use jet_nats::{
    jetstream::{push_subscription::PushSubscription, JetStream},
    Message,
};
use std::time::Duration;

#[derive(Clone)]
pub struct Jet {
    pub jet_stream: JetStream,
}

impl Jet {
    pub async fn new(nats_url: &str) -> Self {
        // TODO: try to use async
        // TODO: add options no reconnect
        let nats = jet_nats::connect(nats_url).expect("Cannot connect to nats!");
        let jet_stream = jet_nats::jetstream::new(nats);
        Self { jet_stream }
    }

    // TODO:
    pub fn add_disconnect_handler(&self, disconnect_handler: ()) -> Self {
        todo!()
    }
}

#[async_trait]
impl Transport for Jet {
    type Error = BaseError<()>;
    type Message = Vec<u8>;
    type MessageId = Message;
    type SubscribeId = PushSubscription;

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        self.jet_stream
            .publish(topic, &message)
            .map_err(|_| BaseError::from(GeneratedError::Nats(NatsError::Send)))?;
        Ok(())
    }

    async fn subscribe_with_timeout<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
        _secs: i32,
    ) -> Result<(), Self::Error> {
        // TODO: Implement timeout (secs variable)
        let _stream_info = self
            .jet_stream
            .add_stream(topic)
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Disconnected)))?;
        let subscription = self
            .jet_stream
            .subscribe(topic)
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Disconnected)))?;

        loop {
            match subscription.next() {
                None => {
                    self.unsubscribe(subscription).await?;
                    break;
                }
                Some(message) => match callback.process(message.data.clone(), message).await {
                    Ok(true) => {}
                    Ok(false) => {
                        self.unsubscribe(subscription).await?;
                        break;
                    }
                    Err(error) => {
                        self.unsubscribe(subscription).await?;
                        return Err(error);
                    }
                },
            }
        }

        Ok(())
    }

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error> {
        let default_timeout_in_secs: i32 = 30;
        self.subscribe_with_timeout(topic, callback, default_timeout_in_secs).await
    }

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error> {
        subscribe_id
            .unsubscribe()
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)))
    }

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error> {
        message_id
            .ack()
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Ok)))
    }

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error> {
        todo!()
    }
}
