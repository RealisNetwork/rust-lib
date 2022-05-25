/// Custom Error type for Realis services
use error_registry::RealisErrors;
use backtrace::Backtrace;

// Want to Serialize and Deserialize?
#[derive(Debug)]
pub struct BaseError<D> {
	pub msg: String,
	// #[serde(rename = "type")]
    // pub error_type: RealisErrors,
    pub trace: Backtrace, // backtrace is not serializable type
    pub data: Option<D>,
    /// Numeric id of `error_type`
    pub status: Option<u32>, // Later will be not optional
}

impl<D> BaseError<D> {
    #[must_use]
    pub fn new(msg: String, data: Option<D>, status: Option<u32>) -> Self {
        let trace = Backtrace::new();
        Self {
            msg: msg,
            trace: trace,
            data: data,
            status: status,
        }
    }
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

// impl<T: serde::Serialize> TryFrom<BaseError<T>> for BaseError<serde_json::Value> {
//     type Error = BaseError<String>;
//     fn try_from(other: BaseError<T>) -> Result<BaseError<serde_json::Value>, Self::Error> {
//         match serde_json::to_value(other.data) {
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
//                 //     data: format!("Data cannot be serialized with: {}", error),
//                 //     status: other.status,
//                 // }
//                 Err(error)
//             }
//         }   
//     }
// }


#[cfg(test)]
mod tests {   

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
