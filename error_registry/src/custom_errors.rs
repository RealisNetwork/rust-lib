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
    pub fn to_u32(&self) -> u32 {
        match self {
            CustomErrorType::Default => 1000u32, //error code, three digits after must be zeros
            CustomErrorType::Blockchain(blockchain) => 2000u32 + blockchain.to_u32(),
            CustomErrorType::Rpc(rpc) => 3000u32 + rpc.to_u32(),
            CustomErrorType::Nats(nats) => 4000u32 + nats.to_u32(),
            CustomErrorType::Db(db) => 5000u32 + db.to_u32(),
            CustomErrorType::EnvLoadedError(env_loaded_error) => 6000u32 + env_loaded_error.to_u32(),
            CustomErrorType::Common(common) => 7000u32 + common.to_u32(),
            CustomErrorType::Utils(utils) => 8000u32 + utils.to_u32(),
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
    pub fn to_u32(&self) -> u32 {
        match self {
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

impl Rpc {
    pub fn to_u32(&self) -> u32 {
        match self {
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

impl Nats {
    pub fn to_u32(&self) -> u32 {
        match self {
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
impl Db {
    pub fn to_u32(&self) -> u32 {
        match self {
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

impl Common {
    pub fn to_u32(&self) -> u32 {
        match self {
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

impl EnvLoadedError {
    pub fn to_u32(&self) -> u32 {
        match self {
            EnvLoadedError::Load => 1u32,
            EnvLoadedError::Convert => 2u32,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Utils {
    Convert, // 777008001
}

impl Utils {
    pub fn to_u32(&self) -> u32 {
        match self {
            Utils::Convert => 1u32,
        }
    }
}