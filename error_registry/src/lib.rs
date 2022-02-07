pub mod traits;

use crate::traits::ToJson;
use convert_case::{Case, Casing};
use derive_more::Display;
use realis_macros::{IntoRealisErrors, RealisErrors, ToJson};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use strum::ParseError;
use strum_macros::EnumString;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Error, Debug, Eq, PartialEq, Clone, Display, RealisErrors, IntoRealisErrors)]
pub enum RealisErrors {
    Db(Db),
    Common(Common),
    AdminOptions(AdminOptions),
    Fs(Fs),
    Bff(Bff),
    Utils(Utils),
    Nats(Nats),
    Rpc(Rpc),
    Validation(Validation),
    TwoFactorAuth(TwoFactorAuth),
    Redis(Redis),
    Billing(Billing),
    ProductRegistry(ProductRegistry),
    Permissions(Permissions),
    Cron(Cron),
    Profile(Profile),
    Roles(Roles),
    GooglePlay(GooglePlay),
    Orchestrator(Orchestrator),
    RestorePassword(RestorePassword),
    Blockchain(Blockchain),
    ProductFactory(ProductFactory),
    Soul(Soul),
    Functions(Functions),
    Referrals(Referrals),
    BytesFormatter(BytesFormatter),
    Status(Status),
    Geo(Geo),
    Action(Action),
    Promo(Promo),
    CustomInt(i32),
    CustomString(String),
}

impl Serialize for RealisErrors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_string())
    }
}

// impl From<RealisErrors> for backoff::Error<RealisErrors> {
//     fn from(error: RealisErrors) -> Self {
//         // TODO decide which errors are critical
//         // use for them `backoff::Error::permanent()`
//         backoff::Error::transient(error)
//     }
// }

impl From<tokio::sync::oneshot::error::RecvError> for RealisErrors {
    fn from(_error: tokio::sync::oneshot::error::RecvError) -> Self {
        RealisErrors::Utils(Utils::Parse)
    }
}

impl From<Vec<u8>> for RealisErrors {
    fn from(_: Vec<u8>) -> Self {
        RealisErrors::Nats(Nats::Send)
    }
}

impl From<backoff::Error<RealisErrors>> for RealisErrors {
    fn from(error: backoff::Error<RealisErrors>) -> Self {
        match error {
            backoff::Error::Permanent(err) | backoff::Error::Transient { err, .. } => err,
        }
    }
}

impl From<std::io::Error> for RealisErrors {
    fn from(_: std::io::Error) -> Self {
        RealisErrors::Utils(Utils::IO)
    }
}

impl From<deadpool_postgres::PoolError> for RealisErrors {
    fn from(_: deadpool_postgres::PoolError) -> Self {
        RealisErrors::Db(Db::Pool)
    }
}

impl From<serde_json::Error> for RealisErrors {
    fn from(_: serde_json::Error) -> Self {
        RealisErrors::Utils(Utils::Parse)
    }
}

impl From<Elapsed> for RealisErrors {
    fn from(_: Elapsed) -> Self {
        RealisErrors::Nats(Nats::MessageReplyTimeout)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Db {
    Select,
    Insert,
    Update,
    InvalidTransaction,
    NotFound,
    WalletNotFound,
    UserIdNotFound,
    Remove,
    Create,
    Save,
    Disconnected,
    ConnectionError,
    AlreadyExists,
    Pool,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Common {
    Unknown,
    InternalServerError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum AdminOptions {
    Update,
    Add,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Fs {
    ReadFile,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Bff {
    InvalidAgent,
    InvalidMethod,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Utils {
    Description,
    HexDecode,
    Convert,
    Parse,
    IO,
    Web3,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Nats {
    Send,
    Receive,
    InternalServiceCall,
    Disconnected,
    AddReconnectHandlerError,
    MessageReplyTimeout,
    Unsubscribe,
    Ok,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Rpc {
    Api,
    BlockNotFound,
    EventsNotFound,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Validation {
    Invalid,
    DoesNotMatchPattern,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum TwoFactorAuth {
    HasEntry,
    InvalidToken,
    ExpiredToken,
    Generate,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Redis {
    NotFound,
    InternalServerError,
    Parse,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Billing {
    UpdateBalanceRecord,
    NotEnoughBalance,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum ProductRegistry {
    InternalError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Permissions {
    NotAllowed,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Cron {
    Create,
    Delete,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Profile {
    AlreadyBanned,
    AlreadySubscribed,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Roles {
    AlreadyHasRole,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum GooglePlay {
    InvalidSubscription,
    InvalidPurchaseStatus,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Orchestrator {
    ZeroAmount,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum RestorePassword {
    ExpiredToken,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Blockchain {
    Storage,
    NotEnoughBalance,
    Send,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum ProductFactory {
    InvalidChance,
    InvalidLimit,
    InvalidProductType,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Soul {
    GetData,
    CallContractMethod,
    TxAlreadySending,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Functions {
    EmptyParams,
    MoreThanOneParam,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Referrals {
    AlreadyHasReferrer,
    AlreadyHasCode,
    UnavailableTransaction,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum BytesFormatter {
    HandshakeAuthToken,
    HandshakeSessionToken,
    HandshakeInvalidVersion,
    InternalError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Status {
    Update,
    Get,
    Delete,
    Add,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Geo {
    InternalError,
    InvalidCountry,
    InvalidContinent,
    InvalidIp,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Action {
    NotCancelable,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Serialize, Display, ToJson, EnumString)]
pub enum Promo {
    CodeExpired,
    CodeNotExists,
    CodeIsAlreadyUsed,
    InternalError,
}

#[cfg(test)]
mod tests {
    use crate::{traits::ToJson, Blockchain, Common, Db, Promo, RealisErrors};
    use convert_case::{Case, Casing};
    use log::info;
    use std::str::FromStr;

    #[test]
    fn test() {
        let error = "blockchain.send";
        let splitted = error.split('.').map(String::from).collect::<Vec<String>>();
        let enum_name = splitted.first().unwrap().to_case(Case::UpperCamel);
        let enum_element = splitted.get(1).unwrap().to_case(Case::UpperCamel);
        let a = match enum_name.as_str() {
            "Blockchain" => RealisErrors::Blockchain(Blockchain::from_str(&enum_element).unwrap()),
            _ => RealisErrors::Common(Common::Unknown),
        };
        println!("{:?}", a)
    }
}
