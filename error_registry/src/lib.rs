//! Implementation of custom error type for Realis.
//! `BaseError` its custom error data structure.
//! `ErrorType` its enum of possible errors
//! what need to be traced for Realis microservices.
use crate::custom_errors::Utils;
use crate::generated_errors::{Critical, Db};
use crate::{
    custom_errors::{Common as CustomCommon, CustomErrorType, EnvLoadedError, Nats as CustomNats},
    generated_errors::{Common, Redis},
};
use backtrace::Backtrace;
use generated_errors::GeneratedError;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::default::Default as StdDefault;
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

pub mod custom_errors;
pub mod generated_errors;

/// BaseError - custom error type
#[derive(Clone, Serialize, Deserialize)]
pub struct BaseError<D: Debug> {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: Option<String>,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: u32,
}

// T - is the Ok() value
// E - is the Err() value
// D - is the data inside BaseError
// ET - is Custom or Generated error
pub trait ProcessError<T, E, D, ET>
where
    E: Debug,
    D: Debug,
    ET: Into<ErrorType>,
{
    fn process_err(self, error: ET) -> Result<T, BaseError<D>>;
}

impl<T, E, D, ET> ProcessError<T, E, D, ET> for Result<T, E>
where
    E: Debug + std::fmt::Display,
    D: Debug,
    ET: Into<ErrorType>,
{
    fn process_err(self, error: ET) -> Result<T, BaseError<D>> {
        self.map_err(|err| {
            let trace = Backtrace::new();
            let error_type = error.into();
            BaseError {
                msg: err.to_string(),
                error_type: error_type.clone(),
                trace: Some(format!("{:?}", trace)),
                data: None,
                status: error_type.into(),
            }
        })
    }
}

impl BaseError<()> {
    /// Create a new `BaseError`
    /// # Arguments
    /// * `msg` - Extra message for explanation of Error
    ///
    /// * `error_type` - Type of Error. `ErrorType` - custom Enum
    ///
    /// * `data` - Data that led to the error. Optional
    ///
    /// * `status` - Code of Error type
    ///
    /// # Examples
    ///
    /// ```
    /// use error_registry::{BaseError, ErrorType};
    /// use error_registry::custom_errors::{CustomErrorType, Nats};
    ///
    /// // BaseError save a error backtrace.
    /// let err = BaseError::<()>::new("Custom message".to_string(), ErrorType::Custom(CustomErrorType::Nats(Nats::Send)));
    /// println!("{}", err.trace);
    /// ```
    #[must_use]
    pub fn new<M: ToString, E: Into<ErrorType>>(msg: M, error_type: E) -> Self {
        let error_type: ErrorType = error_type.into();
        let trace = Backtrace::new();
        BaseError {
            msg: msg.to_string(),
            error_type: error_type.clone(),
            trace: Some(format!("{:?}", trace)),
            data: None,
            status: error_type.into(),
        }
    }
}

impl<D: Debug> BaseError<D> {
    pub fn is_critical(&self) -> bool {
        matches!(
            self.error_type,
            ErrorType::Generated(GeneratedError::Critical(_))
        )
    }
}

impl<D: Debug> Debug for BaseError<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ Error message: {}\nError type: {:?}\nTrace:\n{:?}\nData: {:?}\nStatus: {:?} }}",
            self.msg, self.error_type, self.trace, self.data, self.status
        )
    }
}

impl<D: Debug> Display for BaseError<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1b[93mError message\x1b[0m: {}\n\x1b[93mError type\x1b[0m: {:?}\n",
            self.msg, self.error_type
        )
    }
}

impl<D: Debug> From<tokio::sync::oneshot::error::RecvError> for BaseError<D> {
    fn from(error: tokio::sync::oneshot::error::RecvError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<tokio::time::error::Elapsed> for BaseError<D> {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<ratsio::RatsioError> for BaseError<D> {
    fn from(error: ratsio::RatsioError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<tokio::task::JoinError> for BaseError<D> {
    fn from(error: tokio::task::JoinError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<tokio_postgres::Error> for BaseError<D> {
    fn from(error: tokio_postgres::Error) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<deadpool::managed::PoolError<tokio_postgres::Error>> for BaseError<D> {
    fn from(error: deadpool::managed::PoolError<tokio_postgres::Error>) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<std::net::AddrParseError> for BaseError<D> {
    fn from(error: std::net::AddrParseError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<std::num::ParseIntError> for BaseError<D> {
    fn from(error: std::num::ParseIntError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<std::num::ParseFloatError> for BaseError<D> {
    fn from(error: std::num::ParseFloatError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<hex::FromHexError> for BaseError<D> {
    fn from(error: hex::FromHexError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<openssl::error::ErrorStack> for BaseError<D> {
    fn from(error: openssl::error::ErrorStack) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<dotenv::Error> for BaseError<D> {
    fn from(error: dotenv::Error) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<sqlx::error::Error> for BaseError<D> {
    fn from(error: sqlx::error::Error) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> From<deadpool_postgres::CreatePoolError> for BaseError<D> {
    fn from(error: deadpool_postgres::CreatePoolError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: Some(format!("{:?}", trace)),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> StdDefault for BaseError<D> {
    /// Default BaseError.
    ///
    /// Use only if you explicitly want to get a default error!
    /// # Returns
    /// ```
    /// use backtrace::Backtrace;
    /// use error_registry::custom_errors::CustomErrorType;
    /// use error_registry::{BaseError, ErrorType};
    /// use error_registry::custom_errors::CustomErrorType::Default;
    ///
    /// let trace = Backtrace::new();
    /// let error_type = ErrorType::Custom(CustomErrorType::Default);
    ///
    /// BaseError::<()> {
    ///     msg: String::from("Default error."),
    ///     status: error_type.clone().into(),
    ///     error_type: error_type,
    ///     trace: format!("{:?}", trace),
    ///     data: None,
    /// };
    /// ```
    fn default() -> Self {
        let trace = Backtrace::new();
        let error_type = ErrorType::Generated(GeneratedError::Critical(Critical::Db));
        Self {
            msg: String::from("Default error."),
            status: error_type.clone().into(),
            error_type,
            trace: Some(format!("{:?}", trace)),
            data: None,
        }
    }
}

impl<D: Debug> From<GeneratedError> for BaseError<D> {
    /// Create a `BaseError` by `GeneratedError`
    fn from(error: GeneratedError) -> Self {
        let trace = Backtrace::new();
        let error_type = ErrorType::Generated(error);
        Self {
            msg: format!("{:?}", error_type),
            status: error_type.clone().into(),
            error_type,
            trace: Some(format!("{:?}", trace)),
            data: None,
        }
    }
}

impl From<BaseError<()>> for BaseError<Value> {
    /// Create a `BaseError` by `GeneratedError`
    fn from(error: BaseError<()>) -> BaseError<Value> {
        Self {
            msg: error.msg,
            status: error.status,
            error_type: error.error_type,
            trace: error.trace,
            data: None,
        }
    }
}

impl<D: Debug> From<CustomErrorType> for BaseError<D> {
    /// Create a `BaseError` by `CustomErrorType`
    fn from(error: CustomErrorType) -> Self {
        let trace = Backtrace::new();
        let error_type = ErrorType::Custom(error);
        Self {
            msg: format!("{:?}", error_type),
            status: error_type.clone().into(),
            error_type,
            trace: Some(format!("{:?}", trace)),
            data: None,
        }
    }
}

impl<D: Debug> From<RedisError> for BaseError<D> {
    /// Create a `BaseError` by `RedisError`
    fn from(error: RedisError) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        Self {
            msg,
            trace: Some(format!("{:?}", trace)),
            data: None,
            status: error_type.clone().into(),
            error_type,
        }
    }
}

impl<D: Debug> From<serde_json::Error> for BaseError<D> {
    fn from(serde_error: serde_json::Error) -> Self {
        let trace = Backtrace::new();
        let msg = serde_error.to_string();
        let error_type = ErrorType::from(serde_error);
        Self {
            msg,
            trace: Some(format!("{:?}", trace)),
            data: None,
            status: error_type.clone().into(),
            error_type,
        }
    }
}

impl<D: Debug> From<&'static str> for BaseError<D> {
    fn from(error: &'static str) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::Generated(GeneratedError::Critical(Critical::Db));
        Self {
            msg,
            status: error_type.clone().into(),
            error_type,
            trace: Some(format!("{:?}", trace)),
            data: None,
        }
    }
}

/// All custom types of errors in Realis.
///
/// Custom enum extends manually.
///
/// Generated enum create and extend automatically.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorType {
    Custom(CustomErrorType),
    Generated(GeneratedError),
}

impl From<ErrorType> for u32 {
    fn from(error_type: ErrorType) -> u32 {
        match error_type {
            ErrorType::Custom(custom) => 777000000u32 + u32::from(custom), // Into<u32>>::into(custom)
            ErrorType::Generated(generated) => generated.into(),
        }
    }
}

impl StdDefault for ErrorType {
    fn default() -> Self {
        ErrorType::Custom(CustomErrorType::Default)
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for ErrorType {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        ErrorType::Custom(CustomErrorType::Nats(CustomNats::Receive))
    }
}

impl From<tokio::time::error::Elapsed> for ErrorType {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        // Custom Nats: Disconnected
        ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected))
    }
}

impl From<ratsio::RatsioError> for ErrorType {
    fn from(_: ratsio::RatsioError) -> Self {
        // Custom Nats: Disconnected
        ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected))
    }
}

impl From<deadpool::managed::PoolError<tokio_postgres::Error>> for ErrorType {
    fn from(_: deadpool::managed::PoolError<tokio_postgres::Error>) -> Self {
        // Custom DB: ConnectionError
        ErrorType::Generated(GeneratedError::Critical(Critical::Db))
    }
}
impl From<tokio_postgres::Error> for ErrorType {
    fn from(_: tokio_postgres::Error) -> Self {
        // Custom DB: ConnectionError
        ErrorType::Generated(GeneratedError::Critical(Critical::Db))
    }
}

impl From<openssl::error::ErrorStack> for ErrorType {
    fn from(_: openssl::error::ErrorStack) -> Self {
        // Custom DB: ConnectionError
        ErrorType::Generated(GeneratedError::Critical(Critical::Db))
    }
}

impl From<deadpool_postgres::CreatePoolError> for ErrorType {
    fn from(_: deadpool_postgres::CreatePoolError) -> Self {
        // Custom DB: ConnectionError
        ErrorType::Generated(GeneratedError::Critical(Critical::Db))
    }
}

impl From<sqlx::error::Error> for ErrorType {
    fn from(error: sqlx::error::Error) -> Self {
        match error {
            // Custom DB: ConnectionError
            sqlx::error::Error::Io(_)
            | sqlx::error::Error::PoolClosed
            | sqlx::error::Error::PoolTimedOut
            | sqlx::error::Error::Configuration(_)
            | sqlx::error::Error::Tls(_)
            | sqlx::error::Error::WorkerCrashed => {
                ErrorType::Generated(GeneratedError::Critical(Critical::Db))
            }
            // Generated DB: Invalid Transaction
            sqlx::error::Error::Database(_)
            | sqlx::error::Error::ColumnDecode { .. }
            | sqlx::error::Error::Protocol(_)
            | sqlx::error::Error::TypeNotFound { .. }
            | sqlx::error::Error::ColumnNotFound(_)
            | sqlx::error::Error::ColumnIndexOutOfBounds { .. }
            | sqlx::error::Error::Decode(_)
            | sqlx::error::Error::Migrate(_) => {
                ErrorType::Generated(GeneratedError::Db(Db::InvalidTransaction))
            }
            // Generated DB: Not Found
            sqlx::error::Error::RowNotFound => {
                ErrorType::Generated(GeneratedError::Db(Db::NotFound))
            }
            // Custom: Default
            _ => ErrorType::Custom(CustomErrorType::Default),
        }
    }
}

impl From<dotenv::Error> for ErrorType {
    fn from(_: dotenv::Error) -> Self {
        // Custom EnvLoadedError: Load
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Load))
    }
}

impl From<hex::FromHexError> for ErrorType {
    fn from(_: hex::FromHexError) -> Self {
        // Custom EnvLoadedError: Convert
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<std::num::ParseIntError> for ErrorType {
    fn from(_: std::num::ParseIntError) -> Self {
        // Custom EnvLoadedError: Convert
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<std::num::ParseFloatError> for ErrorType {
    fn from(_: std::num::ParseFloatError) -> Self {
        // Custom EnvLoadedError: Convert
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<std::net::AddrParseError> for ErrorType {
    fn from(_: std::net::AddrParseError) -> Self {
        // Custom EnvLoadedError: Convert
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<std::str::ParseBoolError> for ErrorType {
    fn from(_: std::str::ParseBoolError) -> Self {
        // Custom EnvLoadedError: Convert
        ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<std::str::ParseBoolError> for BaseError<()> {
    fn from(_: std::str::ParseBoolError) -> Self {
        // Custom EnvLoadedError: Convert
        BaseError::from(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
    }
}

impl From<tokio::task::JoinError> for ErrorType {
    fn from(_: tokio::task::JoinError) -> Self {
        ErrorType::Generated(GeneratedError::Common(Common::InternalServerError))
    }
}

impl From<RedisError> for ErrorType {
    /// Case `RedisError` to `ErrorType`
    fn from(_: RedisError) -> Self {
        ErrorType::Generated(GeneratedError::Redis(Redis::InternalServerError))
    }
}

impl From<serde_json::Error> for ErrorType {
    /// Case `serde_json::Error` to `ErrorType`
    fn from(_: serde_json::Error) -> Self {
        ErrorType::Custom(CustomErrorType::Utils(Utils::Parse))
    }
}

impl From<&'static str> for ErrorType {
    /// Case `&'static str` to `ErrorType`
    fn from(_: &'static str) -> Self {
        ErrorType::Custom(CustomErrorType::Common(CustomCommon::ParseString))
    }
}

impl From<std::io::Error> for ErrorType {
    fn from(_: std::io::Error) -> Self {
        ErrorType::Generated(GeneratedError::Common(Common::InternalServerError))
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::custom_errors::Db;
    use crate::generated_errors::{Db as GeneratedDb, GeneratedError, Geo, Utils};
    use crate::{CustomErrorType, ErrorType};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[test]
    fn error_debug_test() {
        let err = BaseError::new(
            "Message text ".to_string(),
            ErrorType::Generated(GeneratedError::Db(GeneratedDb::Select)),
            Some("Data"),
        );
        println!("Debug: \n{:?}", err);
        println!("Display: \n{}", err);
    }

    #[test]
    fn serializing() {
        // Convert to a JSON string.
        let serialized = serde_json::to_string(&GeneratedError::Geo(Geo::InternalError)).unwrap();
        // Prints serialized
        assert_eq!(&serialized.as_str()[1..18], "geo.internalError");
        // println!("serialized = {:#?}", serialized); \"geo.internalError\"
    }

    #[test]
    fn deserializing() {
        // Convert to a JSON string.
        let deserialized = serde_json::from_value::<GeneratedError>(json!("geo.invalidContinent"));
        // Prints serialized
        // assert_eq!(deserialized.unwrap(),
        // GeneratedError::Geo(Geo::InvalidContinent))
        // println!("deserialized = {:#?}", deserialized);
    }

    #[test]
    fn get_code() {
        let generated_code: u32 =
            ErrorType::Generated(GeneratedError::Utils(Utils::Decryption)).into();
        let custom_code: u32 = ErrorType::Custom(CustomErrorType::Db(Db::UserIdNotFound)).into();
        println!(
            "Code for ErrorType::Generated(GeneratedError::Utils(Utils::Decryption)): {}",
            generated_code
        );

        assert_eq!(1148968u32, generated_code);

        println!(
            "Code for ErrorType::Custom(CustomErrorType::Db(Db::UserIdNotFound)): {}",
            custom_code
        );
        assert_eq!(777005004u32, custom_code);
    }

    #[test]
    fn process_error_test() {
        let sample = Result::<(), BaseError<()>>::from(Err(BaseError::from(CustomErrorType::Db(
            Db::UserIdNotFound,
        ))));
        let result: Result<(), BaseError<()>> =
            sample.process_err(GeneratedError::Common(Common::InternalServerError));
        println!("Error: {:#?}", result);
    }
}
