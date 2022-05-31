//! Implementation of custom error type for Realis.
//! `BaseError` its custom error data structure.
//! `ErrorType` its enum of possible errors
//! what need to be traced for Realis microservices.
use std::{
    any::TypeId,
    error::Error,
    fmt,
    fmt::{Debug, Display},
};

use backtrace::Backtrace;

use crate::custom_errors::{CustomErrorType, Nats as CustomNats, Db as CustomDb};
use generated_errors::GeneratedError;

pub mod custom_errors;
pub mod generated_errors;

/// BaseError - custom error type
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BaseError<D> {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>,
}

impl<D> BaseError<D> {
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

impl<D> Display for BaseError<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n", self.msg, self.trace)
    }
}

impl<D, E: 'static + Error> From<E> for BaseError<D> {
    /// Default realization for all structures who implemented
    /// `std::error::Error` trait
    ///
    /// # Examples
    /// ```
    /// use error_registry::{BaseError, ErrorType};
    /// use error_registry::custom_errors::{CustomErrorType, Nats};
    ///
    /// let error = ratsio::error::RatsioError::AckInboxMissing;
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

impl<D> Default for BaseError<D> {
    /// Default BaseError.
    ///
    /// Use only if you explicitly want to get a default error!
    /// # Returns
    /// ```
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

impl<D> From<GeneratedError> for BaseError<D> {
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

impl<D> From<CustomErrorType> for BaseError<D> {
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

impl<E: 'static + Error> From<E> for ErrorType {
    /// Matching `TypeId` of the error type which
    /// implement `std::error:Error` to get relevant
    /// `ErrorType` in the structure `BaseError` without extra code.
    /// To extend list of matching types add it manually.
    fn from(_: E) -> Self {
        if TypeId::of::<E>() == TypeId::of::<tokio::sync::oneshot::error::RecvError>() {
            ErrorType::Custom(CustomErrorType::Nats(CustomNats::Receive))
        } else if TypeId::of::<E>() == TypeId::of::<tokio::time::error::Elapsed>() {
            ErrorType::Custom(CustomErrorType::Nats(CustomNats::Disconnected))
        } else if TypeId::of::<E>() == TypeId::of::<deadpool::managed::PoolError<tokio_postgres::Error>>() {
            ErrorType::Custom(CustomErrorType::Db(CustomDb::ConnectionError))
        } else {
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
    use crate::{CustomErrorType, ErrorType, Nats};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::generated_errors::{GeneratedError, Geo};

    #[test]
    fn error_test() {
        let error = ratsio::error::RatsioError::AckInboxMissing;
        assert_eq!(
            format!("{:?}", BaseError::<()>::from(error).error_type),
            format!("{:?}", ErrorType::Custom(CustomErrorType::Nats(Nats::Send)))
        )
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
}
