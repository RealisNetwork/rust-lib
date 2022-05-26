/// Custom Error type for Realis services
pub mod generated_errors;
use generated_errors::GeneratedError;

// Want to Serialize and Deserialize?
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BaseError<D> {
    pub msg: String,
    #[serde(rename = "err_type")]
    pub error_type: ErrorType,
    pub trace: String,
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>, // Later will be not optional
}

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
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ErrorType {
    Custom(CustomErrorType),
    Generated(GeneratedError),
}

#[serde(untagged)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum CustomErrorType {}

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
