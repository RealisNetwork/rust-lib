use std::fmt::Debug;

use colored::Colorize;
use log::info;

pub struct NatsLoggerWrapper {}

impl NatsLoggerWrapper {
    pub fn got_message<T>(topic: &str, request: T)
        where
            T: Debug,
    {
        info!("By topic - [{:^30}] - got message  - {:?}", topic.purple(), request)
    }

    pub fn sent_message<T>(topic: &str, request: T)
        where
            T: Debug,
    {
        info!("By topic - [{:^30}] - sent message - {:?}", topic.purple(), request)
    }
}
