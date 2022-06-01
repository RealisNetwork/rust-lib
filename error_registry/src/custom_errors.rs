use serde::{Deserialize, Serialize};

#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CustomErrorType {
    Default,
    Blockchain(Blockchain),
    Rpc(Rpc),
    Nats(Nats),
    Db(Db),
    EnvLoadedError(EnvLoadedError),
}

impl From<Blockchain> for CustomErrorType {
    fn from(error: Blockchain) -> Self {
        CustomErrorType::Blockchain(error)
    }
}

impl From<Nats> for CustomErrorType {
    fn from(error: Nats) -> Self {
        CustomErrorType::Nats(error)
    }
}

impl From<Rpc> for CustomErrorType {
    fn from(error: Rpc) -> Self {
        CustomErrorType::Rpc(error)
    }
}

impl From<Db> for CustomErrorType {
    fn from(error: Db) -> Self {
        CustomErrorType::Db(error)
    }
}

impl From<EnvLoadedError> for CustomErrorType {
    fn from(error: EnvLoadedError) -> Self {
        CustomErrorType::EnvLoadedError(EnvLoadedError)
    }
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
    Send,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Db {
    ConnectionError,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EnvLoadedError {
    Load,
    Convert,
}
