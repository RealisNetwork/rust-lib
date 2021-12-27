pub mod traits;

use realis_macros::{RealisErrors, ToJson};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use derive_more::Display;
use crate::error_registry::traits::ToJson;

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, RealisErrors)]
pub enum RealisErrors {
    Db(Db),
    Common(Common),
    AdminOptions(AdminOptions),
    Fs(Fs),
    Bff(Bff),
    Utils(Utils),
    Nats(Nats),
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

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Db {
    Select,
    Insert,
    Update,
    InvalidTransaction,
    NotFound,
    WalletNotFound(String),
    UserIdNotFound(String),
    Remove,
    Create,
    Save,
    Disconnected,
    ConnectionError,
    AlreadyExists,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Common {
    Unknown,
    InternalServerError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum AdminOptions {
    Update,
    Add,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Fs {
    ReadFile,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Bff {
    InvalidAgent,
    InvalidMethod,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Utils {
    Description,
    HexDecode,
    Convert,
    Parse,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Nats {
    Send,
    Receive,
    InternalServiceCall,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Validation {
    Invalid,
    DoesNotMatchPattern,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum TwoFactorAuth {
    HasEntry,
    InvalidToken,
    ExpiredToken,
    Generate,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Redis {
    NotFound,
    InternalServerError,
    Parse,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Billing {
    UpdateBalanceRecord,
    NotEnoughBalance,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum ProductRegistry {
    InternalError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Permissions {
    NotAllowed,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Cron {
    Create,
    Delete,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Profile {
    AlreadyBanned,
    AlreadySubscribed,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Roles {
    AlreadyHasRole,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum GooglePlay {
    InvalidSubscription,
    InvalidPurchaseStatus,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Orchestrator {
    ZeroAmount,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum RestorePassword {
    ExpiredToken,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Blockchain {
    NotEnoughBalance,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum ProductFactory {
    InvalidChance,
    InvalidLimit,
    InvalidProductType,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Soul {
    GetData,
    CallContractMethod,
    TxAlreadySending,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Functions {
    EmptyParams,
    MoreThanOneParam,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Referrals {
    AlreadyHasReferrer,
    AlreadyHasCode,
    UnavailableTransaction,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum BytesFormatter {
    HandshakeAuthToken,
    HandshakeSessionToken,
    HandshakeInvalidVersion,
    InternalError,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Status {
    Update,
    Get,
    Delete,
    Add,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Geo {
    InternalError,
    InvalidCountry,
    InvalidContinent,
    InvalidIp,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Action {
    NotCancelable,
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, Display, ToJson)]
pub enum Promo {
    CodeExpired,
    CodeNotExists,
    CodeIsAlreadyUsed,
    InternalError,
}
