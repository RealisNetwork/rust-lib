use tokio::runtime::Runtime;
use tokio::time::Timeout;
use transport::{ReceivedMessage, Response, StanTransport, Subscription, Transport, VReceivedMessage, VResponse, VSubscription};
use transport::subscription::stan::StanSubscription;
use transport::VReceivedMessage::Stan;

const TOPIC_TEST_1: &str = "test-topic";
const TOPIC_TEST_2: &str = "test-topic-2";
const CLIENT_ID_TEST_1: &str = "test-client-1";
const CLIENT_ID_TEST_2: &str = "test-client-2";
const CLUSTER_ID: &str = "test-cluster";
const NATS_URL: &str = "127.0.0.1:4222";

#[tokio::test(flavor = "multi_thread")]
pub async fn okaying_test() {

    // create Transport as test-client-1
    let mut transport_1 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_1)
        .expect("Fail to init transport_1");
    println!("step 1");

    // new Transport test-client-2
    let mut transport_2 = StanTransport::new(NATS_URL, CLUSTER_ID, CLIENT_ID_TEST_2)
        .expect("Fail to init transport_2");
    println!("step 2");

    let mut vsubscription = transport_1.subscribe(TOPIC_TEST_2).await.unwrap();
    println!("step 3");

    // send as test-client-2 2 messages by topic
    transport_2.publish(VResponse::Response(Response::new(TOPIC_TEST_2, "Hello".as_bytes().to_vec()))).await.unwrap();
    println!("step 4");

    // Assert got messages
    println!("step 5");
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    if let Stan(result) = result {
        assert_eq!(result.message.data, "Hello".as_bytes().to_vec());
        assert!(matches!(result.ok().await, Ok(())));
    }

    transport_2.publish(VResponse::Response(Response::new(TOPIC_TEST_2, "Transport 1".as_bytes().to_vec()))).await.unwrap();
    println!("step 6");

    // Assert got messages
    let result: VReceivedMessage = vsubscription.next().await.unwrap();

    // check messages as client-1 and OK them
    if let Stan(result) = result {
        assert_eq!(result.message.data, "Transport 1".as_bytes().to_vec());
        assert!(matches!(result.ok().await, Ok(())));
    }
    println!("step 7");

    // shutdown client-1
    if let VSubscription::Stan(subs) = vsubscription {
        subs.unsubscribe().await.unwrap();
    }
    println!("step 8");


    // send as test-client-2 1 message by topic
    transport_2.publish(VResponse::Response(Response::new(TOPIC_TEST_2, "Sun".as_bytes().to_vec()))).await.unwrap();

    println!("step 9");

    // create Subscribe as test-client-1
    let mut vsubscription = transport_1.subscribe(TOPIC_TEST_2).await.unwrap();


    // send as test-client-2 2 messages by topic
    transport_2.publish(VResponse::Response(Response::new(TOPIC_TEST_2, "Sun".as_bytes().to_vec()))).await.unwrap();

    // check messages as client-1 and OK them
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    if let Stan(result) = result {
        assert_eq!(result.message.data, "Sun".as_bytes().to_vec());
        assert!(matches!(result.ok().await, Ok(())));
    }

    // check messages as client-1 and OK them
    let result: VReceivedMessage = vsubscription.next().await.unwrap();
    if let Stan(result) = result {
        assert_eq!(result.message.data, "Sun".as_bytes().to_vec());
        assert!(matches!(result.ok().await, Ok(())));
    }


    //
    // // Waiting 30 sec
    // let check_for_messages = tokio::time::timeout(std::time::Duration::from_secs(5), async move {
    //     let result: VReceivedMessage = vsubscription.next().await.unwrap();
    //     if let Stan(result) = result {
    //         assert_eq!(result.message.data, "Sun".as_bytes().to_vec());
    //         assert!(matches!(result.ok().await, Ok(())));
    //     }
    // }).await;
    //
    //
    // assert!(matches!(check_for_messages, Err(_)));
}