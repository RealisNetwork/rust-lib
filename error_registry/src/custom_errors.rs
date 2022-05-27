use serde::{Deserialize, Serialize};

#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CustomErrorType {
    Default,
    Blockchain(Blockchain),
    Rpc(Rpc),
    Nats(Nats),
    Db(Db),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Blockchain {
    Send,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Rpc {
    Api,
    BlockNotFound,
    EventNotFound,
    Disconnected,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Nats {
    Receive,
    Ok,
    Unsubscribe,
    Disconnected,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Db {
    ConnectionError,
}
