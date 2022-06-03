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
    Utils(Utils),
}

impl CustomErrorType {
    pub fn to_u64(&self) -> u64 {
        match self {
            CustomErrorType::Default => 1000u64, //error code, three digits after must be zeros
            CustomErrorType::Blockchain(blockchain) => 2000u64 + blockchain.to_u64(),
            CustomErrorType::Rpc(rpc) => 3000u64 + rpc.to_u64(),
            CustomErrorType::Nats(nats) => 4000u64 + nats.to_u64(),
            CustomErrorType::Db(db) => 5000u64 + db.to_u64(),
            CustomErrorType::EnvLoadedError(env_loaded_error) => 6000u64 + env_loaded_error.to_u64(),
            CustomErrorType::Common(common) => 7000u64 + common.to_u64(),
            CustomErrorType::Utils(utils) => 8000u64 + utils.to_u64(),
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

impl Blockchain {
    pub fn to_u64(&self) -> u64 {
        match self {
            Blockchain::Send => 1u64,
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

impl Rpc {
    pub fn to_u64(&self) -> u64 {
        match self {
            Rpc::Api => 1u64,
            Rpc::BlockNotFound => 2u64,
            Rpc::EventNotFound => 3u64,
            Rpc::Disconnected => 4u64,
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

impl Nats {
    pub fn to_u64(&self) -> u64 {
        match self {
            Nats::Receive => 1u64,
            Nats::Ok => 2u64,
            Nats::Unsubscribe => 3u64,
            Nats::Disconnected => 4u64,
            Nats::Send => 5u64,
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
impl Db {
    pub fn to_u64(&self) -> u64 {
        match self {
            Db::ConnectionError => 1u64,
            Db::WalletNotFound => 2u64,
            Db::AlreadyExists => 3u64,
            Db::UserIdNotFound => 4u64,
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

impl Common {
    pub fn to_u64(&self) -> u64 {
        match self {
            Common::Overflow => 1u64,
            Common::Parse => 2u64,
            Common::ParseString => 3u64,
            Common::ParseInt => 4u64,
            Common::ParseBool => 5u64,
            Common::OutOfRange => 6u64,
            Common::Other => 7u64,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EnvLoadedError {
    Load,
    Convert,
}

impl EnvLoadedError {
    pub fn to_u64(&self) -> u64 {
        match self {
            EnvLoadedError::Load => 1u64,
            EnvLoadedError::Convert => 2u64,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Utils {
    Convert, // 777008001
}

impl Utils {
    pub fn to_u64(&self) -> u64 {
        match self {
            Utils::Convert => 1u64,
        }
    }
}