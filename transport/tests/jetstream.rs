#![feature(async_closure)]
#![feature(option_result_contains)]

const TOPIC_1: &str = "first_topic";
const TOPIC_2: &str = "second_topic";
const MESSAGE_1: &str = "MESSAGE";
const MESSAGE_2: &str = "MESSAGE-2";
const NATS_URL: &str = "127.0.0.1:4222";

#[cfg(test)]
mod jet_stream_test {
    use jet_stream::jetstream::RawStreamMessage;
    use transport::subscription::Subscription;
    use transport::transport::jet_stream::JetTransport;
    use transport::{ReceivedMessage, Response, Transport, VReceivedMessage, VResponse};

    use crate::{MESSAGE_1, MESSAGE_2, NATS_URL, TOPIC_1, TOPIC_2};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[ignore]
    async fn jt_sent_and_rec() {
        let transport_1 = JetTransport::new(NATS_URL).expect("Fail to create transport");

        let transport_2 = JetTransport::new(NATS_URL).expect("Fail to create transport");

        let mut subscription = transport_2
            .subscribe(TOPIC_1)
            .await
            .expect("Fail to subscribe .");

        transport_1
            .publish(VResponse::Response(Response {
                topic_res: TOPIC_1.to_owned(),
                response: serde_json::to_vec(MESSAGE_1).unwrap(),
            }))
            .await
            .expect("Fail to publish");

        let message: VReceivedMessage = subscription.next().await.expect("Fail to get message");

        assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_1);
        message.ok().await.expect("Fail to ack");

        subscription
            .unsubscribe()
            .await
            .expect("Fail to unsubscribe.");

        let mut subscription = transport_1
            .subscribe(TOPIC_1)
            .await
            .expect("Fail to subscribe .");
        let mut subscription_2 = transport_1
            .subscribe(TOPIC_2)
            .await
            .expect("Fail to subscribe .");

        transport_2
            .publish(VResponse::Response(Response {
                topic_res: TOPIC_2.to_owned(),
                response: serde_json::to_vec(MESSAGE_2).unwrap(),
            }))
            .await
            .expect("Fail to publish");

        transport_2
            .publish(VResponse::Response(Response {
                topic_res: TOPIC_1.to_owned(),
                response: serde_json::to_vec(MESSAGE_1).unwrap(),
            }))
            .await
            .expect("Fail to publish");

        let message: VReceivedMessage = subscription.next().await.expect("Fail to get message");
        assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_1);
        message.ok().await.expect("Fail to ack");

        let message: VReceivedMessage = subscription_2.next().await.expect("Fail to get message");
        assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_2);
        message.ok().await.expect("Fail to ack");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore]
    pub async fn jt_sent_and_observed() {
        let transport_1 = JetTransport::new(NATS_URL).expect("Fail to create transport");

        // subscribe to send response on future message
        let mut subscribtion = transport_1
            .subscribe(TOPIC_1)
            .await
            .expect("Fail to subscribe.");

        tokio::task::spawn(async move {
            let message = subscribtion.next().await.expect("Fail to get message");
            assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_1);
            message.ok().await.expect("Fail to ack");
            transport_1
                .publish(VResponse::Response(Response {
                    topic_res: TOPIC_2.to_owned(),
                    response: serde_json::to_vec(MESSAGE_2).unwrap(),
                }))
                .await
                .expect("Fail to send back");
        });

        let transport_2 = JetTransport::new(NATS_URL).expect("Fail to create transport");

        let reply = transport_2
            .send_message_and_observe_reply(
                TOPIC_2.to_owned(),
                VResponse::Response(Response {
                    topic_res: TOPIC_1.to_owned(),
                    response: serde_json::to_vec(MESSAGE_1).unwrap(),
                }),
                None,
            )
            .await
            .expect("Fait to take reply.");

        assert_eq!(reply.deserialize::<String>().unwrap(), MESSAGE_2);
    }

}
