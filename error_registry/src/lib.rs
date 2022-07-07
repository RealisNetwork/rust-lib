//! Implementation of custom error type for Realis.
//! `BaseError` its custom error data structure.
//! `ErrorType` its enum of possible errors
//! what need to be traced for Realis microservices.
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};
use backtrace::Backtrace;
use serde_json::Value;
use crate::{
    custom_errors::{CustomErrorType, EnvLoadedError, Nats as CustomNats},
    generated_errors::Common,
};
use generated_errors::GeneratedError;
use crate::generated_errors::{Critical, Db};

pub mod custom_errors;
pub mod generated_errors;

/// BaseError - custom error type
#[derive(Clone, Serialize, Deserialize)]
pub struct BaseError<D: Debug> {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: u32,
}

impl<D: Debug> BaseError<D> {
    pub fn is_critical(&self) -> bool {
        match self.error_type {
            ErrorType::Generated(GeneratedError::Critical(_)) => true,
            _ => false,
        }
    }
}

impl<D: Debug> BaseError<D> {
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
    /// let err = BaseError::<()>::new("Custom message".to_string(), ErrorType::Custom(CustomErrorType::Nats(Nats::Send)), None);
    /// println!("{}", err.trace);
    /// ```
    #[must_use]
    pub fn new(msg: String, error_type: ErrorType, data: Option<D>) -> Self {
        let trace = Backtrace::new();
        let status = error_type.clone().into();

        Self {
            msg,
            trace: format!("{:?}", trace),
            error_type,
            data,
            status,
        }
    }
}

impl<D: Debug> Debug for BaseError<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ Error message: {}\nError type: {:?}\nTrace:\n{}\nData: {:?}\nStatus: {:?} }}",
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

impl<D: Debug, E: 'static + Error> From<E> for BaseError<D> {
    /// Default realization for all structures who implemented
    /// `std::error::Error` trait
    ///
    /// # Examples
    /// ```
    /// use error_registry::{BaseError, ErrorType};
    /// use error_registry::custom_errors::{CustomErrorType, Nats};
    ///
    /// let error = ratsio::error::RatsioError::AckInboxMissing;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BaseError::<()>::from(error).error_type),
    ///     format!("{:?}", ErrorType::Custom(CustomErrorType::Nats(Nats::Disconnected)))
    /// );
    /// ```
    fn from(error: E) -> Self {
        let trace = Backtrace::new();
        let msg = error.to_string();
        let error_type = ErrorType::from(error);
        BaseError {
            msg,
            trace: format!("{:?}", trace),
            status: error_type.clone().into(),
            error_type,
            data: None,
        }
    }
}

impl<D: Debug> Default for BaseError<D> {
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
        let error_type = ErrorType::Custom(CustomErrorType::Default);
        Self {
            msg: String::from("Default error."),
            status: error_type.clone().into(),
            error_type: error_type,
            trace: format!("{:?}", trace),
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
            error_type: error_type,
            trace: format!("{:?}", trace),
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
            trace: format!("{:?}", trace),
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
}impl From<tokio_postgres::Error> for ErrorType {
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
            | sqlx::error::Error::WorkerCrashed => ErrorType::Generated(GeneratedError::Critical(Critical::Db)),
            // Generated DB: Invalid Transaction
            sqlx::error::Error::Database(_)
            | sqlx::error::Error::ColumnDecode { .. }
            | sqlx::error::Error::Protocol(_)
            | sqlx::error::Error::TypeNotFound { .. }
            | sqlx::error::Error::ColumnNotFound(_)
            | sqlx::error::Error::ColumnIndexOutOfBounds { .. }
            | sqlx::error::Error::Decode(_)
            | sqlx::error::Error::Migrate(_) => ErrorType::Generated(GeneratedError::Db(Db::InvalidTransaction)),
            // Generated DB: Not Found
            sqlx::error::Error::RowNotFound => ErrorType::Generated(GeneratedError::Db(Db::NotFound)),
            // Custom: Default
            _ => ErrorType::Custom(CustomErrorType::Default)
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

impl From<tokio::task::JoinError> for ErrorType {
    fn from(_: tokio::task::JoinError) -> Self {
        ErrorType::Generated(GeneratedError::Common(Common::InternalServerError))
    }
}

impl From<CustomErrorType> for ErrorType {
    /// Cast `CustomErrorType` to `ErrorType`
    fn from(error: CustomErrorType) -> Self {
        ErrorType::Custom(error)
    }
}

impl From<GeneratedError> for ErrorType {
    /// Cast `GeneratedError` to `ErrorType`
    fn from(gen_err: GeneratedError) -> Self {
        ErrorType::Generated(gen_err)
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::{custom_errors::Db, CustomErrorType, ErrorType};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::generated_errors::{Db as GeneratedDb, GeneratedError, Geo, Utils};

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
}
