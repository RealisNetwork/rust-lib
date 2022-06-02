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
    Common(Common),
    Utils(Utils)
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

impl From<Common> for CustomErrorType {
    fn from(error: Common) -> Self {
        CustomErrorType::Common(error)
    }
}

impl From<EnvLoadedError> for CustomErrorType {
    fn from(error: EnvLoadedError) -> Self {
        CustomErrorType::EnvLoadedError(error)
    }
}

impl From<Utils> for CustomErrorType {
    fn from(error: Utils) -> Self {
        CustomErrorType::Utils(error)
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
    WalletNotFound,
    AlreadyExists,
    UserIdNotFound,
}

/// Common error types
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Common {
    Overflow,
    Parse,
    ParseString,
    ParseInt,
    ParseBool,
    OutOfRange,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EnvLoadedError {
    Load,
    Convert,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Utils {
    Convert,
    JoinError,
}
