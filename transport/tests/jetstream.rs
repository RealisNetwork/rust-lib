#![feature(async_closure)]

const TOPIC_1: &str = "topic";
const TOPIC_2: &str = "topic-2";
const MESSAGE_1: &str = "MESSAGE";
const MESSAGE_2: &str = "MESSAGE-2";
const NATS_URL: &str = "127.0.0.1:4222";

#[cfg(test)]
mod jet_stream_test {
    use jet_stream::jetstream::RawStreamMessage;

    use transport::{ReceivedMessage, Response, Transport, VReceivedMessage, VResponse};
    use transport::subscription::Subscription;
    use transport::transport::jet_stream::JetTransport;

    use crate::{MESSAGE_1, MESSAGE_2, NATS_URL, TOPIC_1, TOPIC_2};

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    #[ignore]
    async fn jt_sent_and_rec() {
        let transport_1 = JetTransport::new(NATS_URL).expect("Fail to create transport");

        let transport_2 = JetTransport::new(NATS_URL).expect("Fail to create transport");


        let mut subscription = transport_2.subscribe(TOPIC_1).await.expect("Fail to subscribe .");

        transport_1.publish(VResponse::Response(Response {
            topic_res: TOPIC_1.to_owned(),
            response: serde_json::to_vec(MESSAGE_1).unwrap(),
        }
        )).await
            .expect("Fail to publish");

        let message: VReceivedMessage = subscription.next().await.expect("Fail to get message");
        assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_1);
        message.ok().await.expect("Fail to ack");

        subscription.unsubscribe().await.expect("Fail to unsubscribe.");

        let mut subscription = transport_1.subscribe(TOPIC_1).await.expect("Fail to subscribe .");
        let mut subscription_2 = transport_1.subscribe(TOPIC_2).await.expect("Fail to subscribe .");

        transport_2.publish(VResponse::Response(Response {
            topic_res: TOPIC_2.to_owned(),
            response: serde_json::to_vec(MESSAGE_2).unwrap(),
        }
        )).await
            .expect("Fail to publish");

        transport_2.publish(VResponse::Response(Response {
            topic_res: TOPIC_1.to_owned(),
            response: serde_json::to_vec(MESSAGE_1).unwrap(),
        }
        )).await
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
        let mut subscribtion = transport_1.subscribe(TOPIC_1).await.expect("Fail to subscribe.");

        tokio::task::spawn(async move {
            let message = subscribtion.next().await.expect("Fail to get message");
            assert_eq!(message.deserialize::<String>().unwrap(), MESSAGE_1);
            message.ok().await.expect("Fail to ack");
            transport_1.publish(VResponse::Response(Response {
                topic_res: TOPIC_2.to_owned(),
                response: serde_json::to_vec(MESSAGE_2).unwrap(),
            })).await.expect("Fail to send back");
        }
        );

        let transport_2 = JetTransport::new(NATS_URL).expect("Fail to create transport");


        let reply = transport_2.send_message_and_observe_reply(TOPIC_2.to_owned(), VResponse::Response(Response {
            topic_res: TOPIC_1.to_owned(),
            response: serde_json::to_vec(MESSAGE_1).unwrap(),
        }), None).await.expect("Fait to take reply.");


        assert_eq!(reply.deserialize::<String>().unwrap(), MESSAGE_2);
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // #[ignore]
    pub async fn ok_message() {
        // create Transport as test-client-1
        let mut transport_1 = JetTransport::new(NATS_URL)
            .expect("Fail to init transport_1");

            // new Transport test-client-2
        let mut transport_2 = JetTransport::new(NATS_URL)
            .expect("Fail to init transport_2");

        let mut vsubscription = transport_1.subscribe(TOPIC_2).await.unwrap();

        // send as test-client-2 2 messages by topic
        transport_2
            .publish(VResponse::Response(Response::new(
                TOPIC_2,
                serde_json::to_vec(MESSAGE_2)
                    .unwrap(),
            )))
            .await
            .unwrap();

        // Assert got messages
        let result: VReceivedMessage = vsubscription.next().await.unwrap();
        println!("{:#?}", result.deserialize::<String>().unwrap());
        assert_eq!(
            result.deserialize::<String>().unwrap(),
            MESSAGE_2
        );
        assert!(matches!(result.ok().await, Ok(())));

        // send as test-client-2 2 messages by topic
        transport_2
            .publish(VResponse::Response(Response::new(
                TOPIC_2,
                serde_json::to_vec(MESSAGE_1)
                    .unwrap(),
            )))
            .await
            .unwrap();

        // Assert got messages
        let result: VReceivedMessage = vsubscription.next().await.unwrap();
        println!("{:#?}", result.deserialize::<String>().unwrap());
        assert_eq!(
            result.deserialize::<String>().unwrap(),
            MESSAGE_1
        );
        assert!(matches!(result.ok().await, Ok(())));

        // shutdown client-1 and drop subscription
        drop(vsubscription);
        drop(transport_1);

    // send as test-client-2 1 message by topic
        transport_2
            .publish(VResponse::Response(Response::new(
                TOPIC_2,
                serde_json::to_vec("new message")
                    .unwrap(),
            )))
            .await
            .unwrap();

        // create Transport as test-client-1
        let mut transport_1 = JetTransport::new(NATS_URL)
            .expect("Fail to init transport_1");

        // create Subscribe as test-client-1
        let mut vsubscription = transport_1.subscribe(TOPIC_2).await.unwrap();

        // send as test-client-2 2 messages by topic
        transport_2
            .publish(VResponse::Response(Response::new(
                TOPIC_2,
                serde_json::to_vec("new message 2")
                    .unwrap(),
            )))
            .await
            .unwrap();

        // check messages as client-1 and OK them
        let result: VReceivedMessage = vsubscription.next().await.unwrap();
        println!("{:#?}", result.deserialize::<String>().unwrap());
        assert_eq!(
            result.deserialize::<String>().unwrap(),
            "new message".to_string()
        );
        assert!(matches!(result.ok().await, Ok(())));

        // check messages as client-1 and OK them
        let result: VReceivedMessage = vsubscription.next().await.unwrap();
        println!("{:#?}", result.deserialize::<String>().unwrap());
        assert_eq!(
            result.deserialize::<String>().unwrap(),
            "new message 2".to_string()
        );
        assert!(matches!(result.ok().await, Ok(())));

        let check_messages = vsubscription
            .next_timeout(std::time::Duration::from_secs(10))
            .await;

        assert!(matches!(check_messages, Err(_)));
    }
}


