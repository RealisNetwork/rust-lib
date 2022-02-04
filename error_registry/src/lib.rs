pub mod traits;

use crate::traits::ToJson;
use convert_case::{Case, Casing};
use derive_more::Display;
use realis_macros::{RealisErrors, ToJson};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use strum::ParseError;
use strum_macros::EnumString;
use thiserror::Error;
use tokio::time::error::Elapsed;

#[derive(Error, Debug, Eq, PartialEq, Clone, Display, RealisErrors)]
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

impl<'de> Deserialize<'de> for RealisErrors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let error = String::deserialize(deserializer)?;

        if error.contains('.') {
            let splitted = error.split('.').map(String::from).collect::<Vec<String>>();
            let enum_name = splitted
                .first()
                .ok_or(D::Error::custom("Missing error prefix error!"))?
                .to_case(Case::UpperCamel);
            let element = splitted
                .get(1)
                .ok_or(D::Error::custom("Missing error postfix error!"))?;

            let realis_error = match match_error(enum_name, element) {
                Ok(realis_err) => realis_err,
                Err(_) => RealisErrors::Common(Common::Unknown),
            };
            Ok(realis_error)
        } else {
            Err(D::Error::custom("Unknown error!"))
        }
    }
}

pub fn match_error(enum_name: String, element: &String) -> Result<RealisErrors, ParseError> {
    let enum_element = element.to_case(Case::UpperCamel);
    let realis_error = match enum_name.as_str() {
        "Db" => RealisErrors::Db(Db::from_str(&enum_element)?),
        "Common" => RealisErrors::Common(Common::from_str(&enum_element)?),
        "AdminOptions" => RealisErrors::AdminOptions(AdminOptions::from_str(&enum_element)?),
        "Fs" => RealisErrors::Fs(Fs::from_str(&enum_element)?),
        "Bff" => RealisErrors::Bff(Bff::from_str(&enum_element)?),
        "Utils" => RealisErrors::Utils(Utils::from_str(&enum_element)?),
        "Nats" => RealisErrors::Nats(Nats::from_str(&enum_element)?),
        "Rpc" => RealisErrors::Rpc(Rpc::from_str(&enum_element)?),
        "Validation" => RealisErrors::Validation(Validation::from_str(&enum_element)?),
        "TwoFactorAuth" => RealisErrors::TwoFactorAuth(TwoFactorAuth::from_str(&enum_element)?),
        "Redis" => RealisErrors::Redis(Redis::from_str(&enum_element)?),
        "Billing" => RealisErrors::Billing(Billing::from_str(&enum_element)?),
        "ProductRegistry" => RealisErrors::ProductRegistry(ProductRegistry::from_str(&enum_element)?),
        "Permissions" => RealisErrors::Permissions(Permissions::from_str(&enum_element)?),
        "Cron" => RealisErrors::Cron(Cron::from_str(&enum_element)?),
        "Profile" => RealisErrors::Profile(Profile::from_str(&enum_element)?),
        "Roles" => RealisErrors::Roles(Roles::from_str(&enum_element)?),
        "GooglePlay" => RealisErrors::GooglePlay(GooglePlay::from_str(&enum_element)?),
        "Orchestrator" => RealisErrors::Orchestrator(Orchestrator::from_str(&enum_element)?),
        "RestorePassword" => RealisErrors::RestorePassword(RestorePassword::from_str(&enum_element)?),
        "Blockchain" => RealisErrors::Blockchain(Blockchain::from_str(&enum_element)?),
        "ProductFactory" => RealisErrors::ProductFactory(ProductFactory::from_str(&enum_element)?),
        "Soul" => RealisErrors::Soul(Soul::from_str(&enum_element)?),
        "Functions" => RealisErrors::Functions(Functions::from_str(&enum_element)?),
        "Referrals" => RealisErrors::Referrals(Referrals::from_str(&enum_element)?),
        "BytesFormatter" => RealisErrors::BytesFormatter(BytesFormatter::from_str(&enum_element)?),
        "Status" => RealisErrors::Status(Status::from_str(&enum_element)?),
        "Geo" => RealisErrors::Geo(Geo::from_str(&enum_element)?),
        "Action" => RealisErrors::Action(Action::from_str(&enum_element)?),
        "Promo" => RealisErrors::Promo(Promo::from_str(&enum_element)?),
        "CustomInt" => RealisErrors::CustomInt(element.clone().parse::<i32>().expect("Not a number!")),
        "CustomString" => RealisErrors::CustomString(element.clone()),
        _ => RealisErrors::Common(Common::Unknown),
    };
    Ok(realis_error)
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
