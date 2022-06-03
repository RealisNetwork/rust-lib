//! Implementation of custom error type for Realis.
//! `BaseError` its custom error data structure.
//! `ErrorType` its enum of possible errors
//! what need to be traced for Realis microservices.
use std::{
    any::TypeId,
    error::Error,
    fmt,
    fmt::{write, Debug, Display, Formatter},
};

use backtrace::Backtrace;

use crate::{
    custom_errors::{CustomErrorType, Db as CustomDb, EnvLoadedError, Nats as CustomNats, Utils},
    generated_errors::Common,
};
use generated_errors::GeneratedError;

pub mod custom_errors;
pub mod generated_errors;

/// BaseError - custom error type
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseError<D: Debug> {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>,
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
    /// let err = BaseError::<()>::new("Custom message".to_string(), ErrorType::Custom(CustomErrorType::Nats(Nats::Send)), None, None);
    /// println!("{}", err.trace);
    /// ```
    #[must_use]
    pub fn new(msg: String, error_type: ErrorType, data: Option<D>, status: Option<u32>) -> Self {
        let trace = Backtrace::new();
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
    ///     format!("{:?}", ErrorType::Custom(CustomErrorType::Nats(Nats::Send)))
    /// );
    /// ```
    fn from(error: E) -> Self {
        let trace = Backtrace::new();
        BaseError {
            msg: error.to_string(),
            trace: format!("{:?}", trace),
            // Do not cast error type to ErrorType automatically
            // Need to add error type manually to `from` method of ErrorType
            // If error type not recognized return a default ErrorType
            error_type: ErrorType::from(error),
            data: None,
            status: None,
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
    /// use error_registry::ErrorType;
    ///
    /// let trace = Backtrace::new();
    ///
    /// Self {
    ///     msg: String::from("Default error."),
    ///     error_type: ErrorType::Custom(CustomErrorType::Default),
    ///     trace: format!("{:?}", trace),
    ///     data: None,
    ///     status: None,
    /// }
    /// ```
    fn default() -> Self {
        let trace = Backtrace::new();
        Self {
            msg: String::from("Default error."),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            trace: format!("{:?}", trace),
            data: None,
            status: None,
        }
    }
}

impl<D: Debug> From<GeneratedError> for BaseError<D> {
    /// Create a `BaseError` by `GeneratedError`
    fn from(error: GeneratedError) -> Self {
        let trace = Backtrace::new();
        Self {
            msg: format!("{:?}", error),
            error_type: ErrorType::Generated(error),
            trace: format!("{:?}", trace),
            data: None,
            status: None,
        }
    }
}

impl<D: Debug> From<CustomErrorType> for BaseError<D> {
    /// Create a `BaseError` by `CustomErrorType`
    fn from(error: CustomErrorType) -> Self {
        let trace = Backtrace::new();
        Self {
            msg: format!("{:?}", error),
            error_type: ErrorType::Custom(error),
            trace: format!("{:?}", trace),
            data: None,
            status: None,
        }
    }
}

/// All custom types of errors in Realis.
///
/// Custom enum extends manually.
///
/// Generated enum create and extend automatically.
#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    Custom(CustomErrorType),
    Generated(GeneratedError),
}

impl ErrorType{
    pub fn to_u64(&self) -> u64 {
        match self {
            ErrorType::Custom(custom) => 777000000u64 + custom.to_u64(),
            ErrorType::Generated(generated) => generated.to_u64(),
        }
    }
}

impl<E: 'static + Error> From<E> for ErrorType {
    /// Matching `TypeId` of the error type which
    /// implement `std::error:Error` to get relevant
    /// `ErrorType` in the structure `BaseError` without extra code.
    /// To extend list of matching types add it manually.
    fn from(_: E) -> Self {
        let type_id = TypeId::of::<E>();
        if type_id == TypeId::of::<tokio::sync::oneshot::error::RecvError>() {
            // Custom Nats: Receive
            ErrorType::Custom(CustomErrorType::Nats(CustomNats::Receive))
        } else if (type_id == TypeId::of::<tokio::time::error::Elapsed>()) || (type_id == TypeId::of::<ratsio::RatsioError>()) {
            // Custom Nats: Disconnected
            ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected))
        } else if (type_id == TypeId::of::<deadpool::managed::PoolError<tokio_postgres::Error>>())
            || (type_id == TypeId::of::<tokio_postgres::Error>())
            || (type_id == TypeId::of::<openssl::error::ErrorStack>())
            || (type_id == TypeId::of::<deadpool_postgres::CreatePoolError>())
        {
            // Custom DB: ConnectionError
            ErrorType::Custom(CustomErrorType::Db(CustomDb::ConnectionError))
        } else if type_id == TypeId::of::<dotenv::Error>() {
            // Custom EnvLoadedError: Load
            ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Load))
        } else if (type_id == TypeId::of::<hex::FromHexError>())
            || (type_id == TypeId::of::<std::num::ParseIntError>())
            || (type_id == TypeId::of::<std::net::AddrParseError>())
            || (type_id == TypeId::of::<std::str::ParseBoolError>())
        {
            // Custom EnvLoadedError: Convert
            ErrorType::Custom(CustomErrorType::EnvLoadedError(EnvLoadedError::Convert))
        } else if type_id == TypeId::of::<tokio::task::JoinError>() {
            ErrorType::Generated(GeneratedError::Common(Common::InternalServerError))
        } else {
            // Custom: Default
            ErrorType::Custom(CustomErrorType::Default)
        }
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
            Some(10),
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
        let generated_code: u64 = ErrorType::Generated(GeneratedError::Utils(Utils::Decryption)).to_u64();
        let custom_code: u64 = ErrorType::Custom(CustomErrorType::Db(Db::UserIdNotFound)).to_u64();
        println!("Code for ErrorType::Generated(GeneratedError::Utils(Utils::Decryption)): {}", generated_code);

        assert_eq!(1148968u64, generated_code);

        println!("Code for ErrorType::Custom(CustomErrorType::Db(Db::UserIdNotFound)): {}", custom_code);
        assert_eq!(777005004u64, custom_code);
    }
}
