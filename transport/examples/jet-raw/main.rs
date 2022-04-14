use std::{thread, time::Duration};

const TOPIC: &str = "test-jet-stream-topic";
const NATS_URL: &str = "127.0.0.1:4222";

fn main() {
    // Spawn thread to wait and publish some message
    let publish_thread_join_handle = thread::spawn(move || {
        const THREAD_PREFIX: &str = "PUBLISH";

        let time_to_sleep = 5;
        println!("[{:^15}] - Sleep for a {} seconds...", THREAD_PREFIX, time_to_sleep);
        thread::sleep(Duration::from_secs(time_to_sleep));

        println!("[{:^15}] - Waked up!", THREAD_PREFIX);

        println!("[{:^15}] - Connecting to nats...", THREAD_PREFIX);
        match jet_nats::connect(NATS_URL) {
            Ok(nats) => {
                println!("[{:^15}] - Successfully connected to nats!", THREAD_PREFIX);

                let jetstream = jet_nats::jetstream::new(nats);

                let topic = TOPIC;
                let message = serde_json::to_vec("Hello, World!").unwrap();

                println!(
                    "[{:^15}] - Publishing by topic: `{}`, message: `{:?}`",
                    THREAD_PREFIX, topic, message
                );
                match jetstream.publish(topic, message) {
                    Ok(publish_ack) => {
                        println!(
                            "[{:^15}] - Successfully published to nats! Some publish info: {:#?}",
                            THREAD_PREFIX, publish_ack
                        );
                    }
                    Err(error) => {
                        println!("[{:^15}] - Fail to publish to nats: {:?}", THREAD_PREFIX, error);
                    }
                }
            }
            Err(error) => {
                println!("[{:^15}] - Fail to connect to nats: {:?}", THREAD_PREFIX, error);
            }
        }
    });

    // Spawn thread to wait and subscribe by topic
    let subscribe_thread_join_handle = thread::spawn(move || {
        const THREAD_PREFIX: &str = "SUBSCRIBE";

        let time_to_sleep = 8;
        println!("[{:^15}] - Sleep for a {} seconds...", THREAD_PREFIX, time_to_sleep);
        thread::sleep(Duration::from_secs(time_to_sleep));

        println!("[{:^15}] - Waked up!", THREAD_PREFIX);

        println!("[{:^15}] - Connecting to nats...", THREAD_PREFIX);
        match jet_nats::connect(NATS_URL) {
            Ok(nats) => {
                println!("[{:^15}] - Successfully connected to nats!", THREAD_PREFIX);

                let jetstream = jet_nats::jetstream::new(nats);

                println!("[{:^15}] - Add stream with name: `{}`", THREAD_PREFIX, TOPIC);
                // Stream name must be same as topic to
                match jetstream.add_stream(TOPIC) {
                    Ok(stream_info) => {
                        println!(
                            "[{:^15}] - Successfully created stream to nats! Some stream info: {:#?}",
                            THREAD_PREFIX, stream_info
                        );
                        let subscription = jetstream.subscribe(TOPIC).unwrap();
                        let message = subscription.next().unwrap();
                        println!("[{:^15}] - Got message: {:#?}", THREAD_PREFIX, message);

                        let data = serde_json::from_slice::<String>(&message.data).unwrap();
                        println!("[{:^15}] - Message data: {:?}", THREAD_PREFIX, data);
                    }
                    Err(error) => {
                        println!("[{:^15}] - Fail to publish to nats: {:?}", THREAD_PREFIX, error);
                    }
                }
            }
            Err(error) => {
                println!("[{:^15}] - Fail to connect to nats: {:?}", THREAD_PREFIX, error);
            }
        }
    });

    let _res = publish_thread_join_handle.join();
    let _res = subscribe_thread_join_handle.join();
}
