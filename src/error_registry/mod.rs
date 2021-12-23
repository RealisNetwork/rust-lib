use realis_macros::RealisErrors;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
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

impl Display for RealisErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Db {
    Select,
    Insert,
    Update,
    InvalidTransaction,
    NotFound,
    Remove,
    Create,
    Save,
}

impl Display for Db {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Common {
    Unknown,
    InternalServerError,
}

impl Display for Common {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum AdminOptions {
    Update,
    Add,
}

impl Display for AdminOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Fs {
    ReadFile,
}

impl Display for Fs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Bff {
    InvalidAgent,
    InvalidMethod,
}

impl Display for Bff {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Utils {
    Description,
}

impl Display for Utils {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Nats {
    Send,
    Receive,
    InternalServiceCall,
}

impl Display for Nats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Validation {
    Invalid,
    DoesNotMatchPattern,
}

impl Display for Validation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum TwoFactorAuth {
    HasEntry,
    InvalidToken,
    ExpiredToken,
    Generate,
}

impl Display for TwoFactorAuth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Redis {
    NotFound,
    InternalServerError,
    Parse,
}

impl Display for Redis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Billing {
    UpdateBalanceRecord,
    NotEnoughBalance,
}

impl Display for Billing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum ProductRegistry {
    InternalError,
}

impl Display for ProductRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Permissions {
    NotAllowed,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Cron {
    Create,
    Delete,
}

impl Display for Cron {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Profile {
    AlreadyBanned,
    AlreadySubscribed,
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Roles {
    AlreadyHasRole,
}

impl Display for Roles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum GooglePlay {
    InvalidSubscription,
    InvalidPurchaseStatus,
}

impl Display for GooglePlay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Orchestrator {
    ZeroAmount,
}

impl Display for Orchestrator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum RestorePassword {
    ExpiredToken,
}

impl Display for RestorePassword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Blockchain {
    NotEnoughBalance,
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum ProductFactory {
    InvalidChance,
    InvalidLimit,
    InvalidProductType,
}

impl Display for ProductFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Soul {
    GetData,
    CallContractMethod,
    TxAlreadySending,
}

impl Display for Soul {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Functions {
    EmptyParams,
    MoreThanOneParam,
}

impl Display for Functions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Referrals {
    AlreadyHasReferrer,
    AlreadyHasCode,
    UnavailableTransaction,
}

impl Display for Referrals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum BytesFormatter {
    HandshakeAuthToken,
    HandshakeSessionToken,
    HandshakeInvalidVersion,
    InternalError,
}

impl Display for BytesFormatter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Status {
    Update,
    Get,
    Delete,
    Add,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Geo {
    InternalError,
    InvalidCountry,
    InvalidContinent,
    InvalidIp,
}

impl Display for Geo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Action {
    NotCancelable,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize, RealisErrors)]
pub enum Promo {
    CodeExpired,
    CodeNotExists,
    CodeIsAlreadyUsed,
    InternalError,
}

impl Display for Promo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
