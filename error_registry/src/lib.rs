/// Custom Error type for Realis services
pub mod generated_errors;
use backtrace::Backtrace;
use deadpool_postgres::tokio_postgres;
use generated_errors::GeneratedError;
use log::error;
use tokio::time::error::Elapsed;

// Want to Serialize and Deserialize?
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BaseError<D> {
    pub msg: String,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>, // Later will be not optional
}

impl<D> BaseError<D> {
    #[must_use]
    pub fn new(msg: String, data: Option<D>, status: Option<u32>, error_type: ErrorType) -> Self {
        let trace = Backtrace::new();
        Self {
            msg: msg,
            trace: format!("{:?}", trace),
            error_type,
            data: data,
            status: status,
        }
    }
}

impl<D> From<tokio::sync::oneshot::error::RecvError> for BaseError<D> {
    fn from(_error: tokio::sync::oneshot::error::RecvError) -> Self {
        BaseError {
            msg: "Cannot parse value".to_string(),
            trace: "".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            data: None,
            status: None,
        }
    }
}

impl<D> From<Elapsed> for BaseError<D> {
    fn from(_: Elapsed) -> Self {
        Self {
            msg: "Message reply timeout".to_string(),
            trace: "".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            data: None,
            status: None,
        }
    }
}

impl<D> From<Vec<u8>> for BaseError<D> {
    fn from(_: Vec<u8>) -> Self {
        BaseError {
            msg: "Message reply timeout".to_string(),
            trace: "".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            data: None,
            status: None,
        }
    }
}

impl<D> From<std::io::Error> for BaseError<D> {
    fn from(_: std::io::Error) -> Self {
        Self {
            msg: "std::io::Error".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            trace: "".to_string(),
            data: None,
            status: None
        }
    }
}

impl<D> From<deadpool_postgres::PoolError> for BaseError<D> {
    fn from(_: deadpool_postgres::PoolError) -> Self {
        Self {
            msg: "deadpool_postgres::PoolError".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            trace: "".to_string(),
            data: None,
            status: None
        }
    }
}

impl<D> Default for BaseError<D> {
    fn default() -> Self {
        Self {
            msg: "Unknown".to_string(),
            error_type: ErrorType::Custom(CustomErrorType::Default),
            trace: "".to_string(),
            data: None,
            status: None
        }
    }
}

// impl<D> From<deadpool::managed::errors::PoolError<tokio_postgres::Error>> for BaseError<D> {
//     fn from(_error: deadpool::managed::errors::PoolError<tokio_postgres::Error>) -> Self {
//         BaseError {
//             msg: format!("{:?}", _error),
//             trace: "".to_string(),
//             error_type: ErrorType::Custom(CustomErrorType::Default),
//             data: None,
//             status: None,
//         }
//     }
// }

// From<deadpool::managed::errors::PoolError<tokio_postgres::Error>>
// impl<T> From<()> for BaseError<T> {
//     fn from(other: BaseError<()>) -> BaseError<T> {
//         Self {
//             msg: other.msg,
//             // error_type: other.error_type,
//             trace: other.trace,
//             data: None,
//             status: other.status,
//         }
//     }
// }

// impl<T: serde::Serialize> TryFrom<BaseError<T>> for
// BaseError<serde_json::Value> {     type Error = BaseError<String>;
//     fn try_from(other: BaseError<T>) -> Result<BaseError<serde_json::Value>,
// Self::Error> {         match serde_json::to_value(other.data) {
//             Ok(value) => {
//                 Ok(Self {
//                     msg: other.msg,
//                     // error_type: other.error_type,
//                     trace: other.trace,
//                     data: Some(value),
//                     status: other.status,
//                 })
//             }
//             Err(error) => {
//                 // Self {
//                 //     msg: other.msg,
//                 //     error_type: other.error_type,
//                 //     trace: other.trace,
//                 //     data: format!("Data cannot be serialized with: {}",
// error),                 //     status: other.status,
//                 // }
//                 Err(error)
//             }
//         }
//     }
// }

#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    Custom(CustomErrorType),
    Generated(GeneratedError),
}

#[serde(untagged)]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum CustomErrorType {
    Default,
}

#[cfg(test)]
mod tests {

    use crate::generated_errors::{GeneratedError, Geo};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

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
