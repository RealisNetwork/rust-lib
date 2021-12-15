use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Common {
    Unknown,
    InternalServerError,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum AdminOptions {
    Update,
    Add,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Fs {
    ReadFile,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Bff {
    InvalidAgent,
    InvalidMethod,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Utils {
    Description,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Nats {
    Send,
    Receive,
    InternalServiceCall,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Validation {
    Invalid,
    DoesNotMatchPattern,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum TwoFactorAuth {
    HasEntry,
    InvalidToken,
    ExpiredToken,
    Generate,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Redis {
    NotFound,
    InternalServerError,
    Parse,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Billing {
    UpdateBalanceRecord,
    NotEnoughBalance,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum ProductRegistry {
    InternalError,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Permissions {
    NotAllowed,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Cron {
    Create,
    Delete,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Profile {
    AlreadyBanned,
    AlreadySubscribed,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Roles {
    AlreadyHasRole,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum GooglePlay {
    InvalidSubscription,
    InvalidPurchaseStatus,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Orchestrator {
    ZeroAmount,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum RestorePassword {
    ExpiredToken,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Blockchain {
    NotEnoughBalance,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum ProductFactory {
    InvalidChance,
    InvalidLimit,
    InvalidProductType,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Soul {
    GetData,
    CallContractMethod,
    TxAlreadySending,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Functions {
    EmptyParams,
    MoreThanOneParam,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Referrals {
    AlreadyHasReferrer,
    AlreadyHasCode,
    UnavailableTransaction,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum BytesFormatter {
    HandshakeAuthToken,
    HandshakeSessionToken,
    HandshakeInvalidVersion,
    InternalError,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Status {
    Update,
    Get,
    Delete,
    Add,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Geo {
    InternalError,
    InvalidCountry,
    InvalidContinent,
    InvalidIp,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Action {
    NotCancelable,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum Promo {
    CodeExpired,
    CodeNotExists,
    CodeIsAlreadyUsed,
    InternalError,
}
