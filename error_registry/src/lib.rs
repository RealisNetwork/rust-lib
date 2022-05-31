use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};

use backtrace::Backtrace;



use generated_errors::GeneratedError;

use crate::custom_errors::CustomErrorType;

pub mod custom_errors;
/// Custom Error type for Realis services
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
    /// * `data` - Data that led to the error
    ///
    /// * `status` - Code of Error type
    ///
    /// # Examples
    ///
    /// ```
    /// use error_registry::{BaseError, ErrorType};
    /// use error_registry::generated_errors::Blockchain;
    /// use error_registry::generated_errors::Blockchain::NotEnoughBalance;
    /// use error_registry::generated_errors::GeneratedError::Blockchain;
    ///
    /// // BaseError save a error backtrace.
    /// let err = BaseError::new("Custom message".to_string(), ErrorType::Generated(Blockchain(NotEnoughBalance)), None, None);
    /// println!(err.trace);
    ///
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

/// Default realization for all structures who implemented `Error` trait
impl<D, E: Error> From<E> for BaseError<D> {
    fn from(error: E) -> Self {
        let trace = Backtrace::new();
        BaseError {
            msg: error.to_string(),
            trace: format!("{:?}", trace),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            data: None,
            status: None,
        }
    }
}


impl<D> Default for BaseError<D> {
    fn default() -> Self {
        let trace = Backtrace::new();
        Self {
            msg: "Error type not recognized. Default error.".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            trace: format!("{:?}", trace),
            data: None,
            status: None,
        }
    }
}

impl<D> From<GeneratedError> for BaseError<D> {
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

#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    Custom(CustomErrorType),
    Generated(GeneratedError),
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    use crate::generated_errors::{GeneratedError, Geo};

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
        assert_eq!(deserialized.unwrap(), GeneratedError::Geo(Geo::InvalidContinent))
        // println!("deserialized = {:#?}", deserialized);
    }

    #[test]
    fn test_deserialize_base_error_with_no_data() {
        // Should be success with BaseError.data == None
        todo!();
    }

    #[test]
    fn test_deserialize_base_error_with_null_data() {
        // Should be success with BaseError.data == None
        todo!();
    }

    #[test]
    fn test_deserialize_base_error_with_data() {
        // Should be success with BaseError.data == Some
        todo!();
    }
}
