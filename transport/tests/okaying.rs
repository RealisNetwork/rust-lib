use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::time::Timeout;
use transport::subscription::stan::StanSubscription;
use transport::VReceivedMessage::Stan;
use transport::{
    ReceivedMessage, Response, StanTransport, Subscription, Transport, VReceivedMessage, VResponse,
    VSubscription,
};

const TOPIC_TEST_1: &str = "test-topic";
const TOPIC_TEST_2: &str = "test-topic-2";
const CLIENT_ID_TEST_1: &str = "test-client-1";
const CLIENT_ID_TEST_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schema {
    msg: String,
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn okaying_test() {
    // create Transport as test-client-1
    let mut transport_1 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_1)
        .expect("Fail to init transport_1");

    // new Transport test-client-2
    let mut transport_2 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_2)
        .expect("Fail to init transport_2");

    let mut vsubscription = transport_1.subscribe(TOPIC_TEST_2).await.unwrap();

    // send as test-client-2 2 messages by topic
    transport_2
        .publish(VResponse::Response(Response::new(
            TOPIC_TEST_2,
            serde_json::to_vec(&Schema {
                msg: "Hello".to_string(),
            })
            .unwrap(),
        )))
        .await
        .unwrap();

    // Assert got messages
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    println!("{:#?}", result.deserialize::<Schema>().unwrap());
    assert_eq!(
        result.deserialize::<Schema>().unwrap().msg,
        "Hello".to_string()
    );
    assert!(matches!(result.ok().await, Ok(())));

    // send as test-client-2 2 messages by topic
    transport_2
        .publish(VResponse::Response(Response::new(
            TOPIC_TEST_2,
            serde_json::to_vec(&Schema {
                msg: "Transport 1".to_string(),
            })
            .unwrap(),
        )))
        .await
        .unwrap();

    // Assert got messages
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    println!("{:#?}", result.deserialize::<Schema>().unwrap());
    assert_eq!(
        result.deserialize::<Schema>().unwrap().msg,
        "Transport 1".to_string()
    );
    assert!(matches!(result.ok().await, Ok(())));

    // shutdown client-1 and drop subscription
    drop(vsubscription);
    drop(transport_1);

    // send as test-client-2 1 message by topic
    transport_2
        .publish(VResponse::Response(Response::new(
            TOPIC_TEST_2,
            serde_json::to_vec(&Schema {
                msg: "Sun".to_string(),
            })
            .unwrap(),
        )))
        .await
        .unwrap();

    // create Transport as test-client-1
    let mut transport_1 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_1)
        .expect("Fail to init transport_1");

    // create Subscribe as test-client-1
    let mut vsubscription = transport_1.subscribe(TOPIC_TEST_2).await.unwrap();

    // send as test-client-2 2 messages by topic
    transport_2
        .publish(VResponse::Response(Response::new(
            TOPIC_TEST_2,
            serde_json::to_vec(&Schema {
                msg: "Sun".to_string(),
            })
            .unwrap(),
        )))
        .await
        .unwrap();

    // check messages as client-1 and OK them
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    println!("{:#?}", result.deserialize::<Schema>().unwrap());
    assert_eq!(
        result.deserialize::<Schema>().unwrap().msg,
        "Sun".to_string()
    );
    assert!(matches!(result.ok().await, Ok(())));

    // check messages as client-1 and OK them
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    println!("{:#?}", result.deserialize::<Schema>().unwrap());
    assert_eq!(
        result.deserialize::<Schema>().unwrap().msg,
        "Sun".to_string()
    );
    assert!(matches!(result.ok().await, Ok(())));

    let check_messages = vsubscription
        .next_timeout(std::time::Duration::from_secs(10))
        .await;

    assert!(matches!(check_messages, Err(_)));
}
