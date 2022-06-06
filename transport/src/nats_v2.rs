use crate::traits::{MessageReceiver, Transport};
use async_nats::Subscriber;
use async_trait::async_trait;
use nats_not_async::{Connection, Message, Subscription};
use std::{future::Future, io::ErrorKind::TimedOut, sync::Arc, time::Duration};
use tokio::time::{error::Elapsed, timeout, Timeout};

use error_registry::{
    custom_errors::{CustomErrorType, Nats as CustomNatsError},
    generated_errors::{GeneratedError, Nats, Nats as GeneratedNatsError},
    BaseError, ErrorType,
};

#[derive(Clone)]
pub struct Nats_v2 {
    pub client: Connection,
}

impl Nats_v2 {
    pub fn new(nats_opts: &str, client_id: &str) -> Result<Self, BaseError<()>> {
        let client = nats_not_async::Options::new()
            .with_name(client_id)
            .connect(format!("nats://{}", nats_opts))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Transport for Nats_v2 {
    type Error = BaseError<()>;
    type Message = Vec<u8>;
    type MessageId = Message;
    type SubscribeId = Subscription;

    async fn publish(&self, topic: &str, message: Self::Message, _topic_res: Option<String>) -> Result<(), Self::Error> {
        if _topic_res.is_none() {
            self.client
                .publish(topic, message)
                .map_err(|_| BaseError::from(GeneratedError::Nats(GeneratedNatsError::Send)))
        } else {
            self.client
                .publish_request(topic, _topic_res.unwrap().as_str(), message)
                .map_err(|_| BaseError::<()>::from(GeneratedError::Nats(GeneratedNatsError::Send)))
        }
    }

    async fn subscribe<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
    ) -> Result<(), Self::Error> {
        let sub = self.client.subscribe(topic)?;

        loop {
            match sub.next() {
                Some(msg) => match callback.process(msg.data.clone(), msg).await {
                    Ok(true) => {}
                    Ok(false) => {
                        sub.unsubscribe();
                        break;
                    }
                    Err(error) => {
                        sub.unsubscribe();
                        return Err(error);
                    }
                },
                None => {
                    sub.unsubscribe();
                    return Err(BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)));
                }
            }
        }
        Ok(())
    }

    async fn subscribe_with_timeout<'a>(
        &self,
        topic: &str,
        callback: impl MessageReceiver<Self::Message, Self::MessageId, Self::Error> + 'a,
        _secs: i32,
    ) -> Result<(), Self::Error> {
        let sub = self.client.subscribe(topic)?;

        loop {
            match sub.next_timeout(Duration::from_secs(_secs as u64)) {
                Ok(msg) => match callback.process(msg.data.clone(), msg).await {
                    Ok(true) => {}
                    Ok(false) => {
                        sub.unsubscribe();
                        break;
                    }
                    Err(error) => {
                        sub.unsubscribe();
                        return Err(error);
                    }
                },
                Err(err) => {
                    sub.unsubscribe();
                    return Err(BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)));
                }
            }
        }
        Ok(())
    }

    async fn unsubscribe(&self, subscribe_id: Self::SubscribeId) -> Result<(), Self::Error> {
        subscribe_id
            .unsubscribe()
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)))
    }

    async fn message_reply(
        &self,
        topic: &str,
        topic_res: &str,
        message: Self::Message,
        duration: Option<Duration>,
    ) -> Result<Self::Message, Self::Error> {
        let sub = self
            .client
            .subscribe(topic_res)
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Disconnected)))?;

        self.publish(topic, message, None).await?;

        let timeout_dur = duration.unwrap_or(Duration::from_secs(25));

        let option_message = timeout(timeout_dur, async { sub.next() }).await?;

        let message = option_message.ok_or(BaseError::from(CustomErrorType::Nats(CustomNatsError::Receive)))?;

        self.ok(message.clone()).await;

        self.unsubscribe(sub).await;

        Ok(message.data)
    }

    async fn ok(&self, message_id: Self::MessageId) -> Result<(), Self::Error> {
        message_id
            .ack()
            .map_err(|_| BaseError::from(CustomErrorType::Nats(CustomNatsError::Unsubscribe)))
    }
}
