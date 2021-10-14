use std::fmt::Debug;

use log::info;

pub struct NatsLoggerWrapper {}

impl NatsLoggerWrapper {
    pub fn got_message<T>(topic: &str, request: T)
    where T: Debug {
        info!("By topic - {:^30} - got message  - {:?}", topic, request)
    }

    pub fn sent_message<T>(topic: &str, request: T)
        where T: Debug {
        info!("By topic - {:^30} - sent message - {:?}", topic, request)
    }
}