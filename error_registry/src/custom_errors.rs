use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CustomErrorType {
    Default,
    Blockchain(Blockchain),
    Rpc(Rpc),
    Nats(Nats),
    Db(Db),
    EnvLoadedError(EnvLoadedError),
    Common(Common),
    Utils(Utils),
}

impl From<CustomErrorType> for u32 {
    fn from(error_type: CustomErrorType) -> u32 {
        match error_type {
            CustomErrorType::Default => 1000u32, // error code, three digits after must be zeros
            CustomErrorType::Blockchain(blockchain) => 2000u32 + u32::from(blockchain),
            CustomErrorType::Rpc(rpc) => 3000u32 + u32::from(rpc),
            CustomErrorType::Nats(nats) => 4000u32 + u32::from(nats),
            CustomErrorType::Db(db) => 5000u32 + u32::from(db),
            CustomErrorType::EnvLoadedError(env_loaded_error) => {
                6000u32 + u32::from(env_loaded_error)
            }
            CustomErrorType::Common(common) => 7000u32 + u32::from(common),
            CustomErrorType::Utils(utils) => 8000u32 + u32::from(utils),
        }
    }
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

impl From<Blockchain> for u32 {
    fn from(error_type: Blockchain) -> u32 {
        match error_type {
            Blockchain::Send => 1u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Rpc {
    Api,
    BlockNotFound,
    EventNotFound,
    Disconnected,
}

impl From<Rpc> for u32 {
    fn from(error_type: Rpc) -> u32 {
        match error_type {
            Rpc::Api => 1u32,
            Rpc::BlockNotFound => 2u32,
            Rpc::EventNotFound => 3u32,
            Rpc::Disconnected => 4u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Nats {
    Receive,
    Ok,
    Unsubscribe,
    Disconnected,
    Send,
}

impl From<Nats> for u32 {
    fn from(error_type: Nats) -> u32 {
        match error_type {
            Nats::Receive => 1u32,
            Nats::Ok => 2u32,
            Nats::Unsubscribe => 3u32,
            Nats::Disconnected => 4u32,
            Nats::Send => 5u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Db {
    ConnectionError,
    WalletNotFound,
    AlreadyExists,
    UserIdNotFound,
}
impl From<Db> for u32 {
    fn from(error_type: Db) -> u32 {
        match error_type {
            Db::ConnectionError => 1u32,
            Db::WalletNotFound => 2u32,
            Db::AlreadyExists => 3u32,
            Db::UserIdNotFound => 4u32,
        }
    }
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

impl From<Common> for u32 {
    fn from(error_type: Common) -> u32 {
        match error_type {
            Common::Overflow => 1u32,
            Common::Parse => 2u32,
            Common::ParseString => 3u32,
            Common::ParseInt => 4u32,
            Common::ParseBool => 5u32,
            Common::OutOfRange => 6u32,
            Common::Other => 7u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EnvLoadedError {
    Load,
    Convert,
}

impl From<EnvLoadedError> for u32 {
    fn from(error_type: EnvLoadedError) -> u32 {
        match error_type {
            EnvLoadedError::Load => 1u32,
            EnvLoadedError::Convert => 2u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Utils {
    Convert, // 777008001
    Parse,
}

impl From<Utils> for u32 {
    fn from(error_type: Utils) -> u32 {
        match error_type {
            Utils::Convert => 1u32,
            Utils::Parse => 2u32,
        }
    }
}
