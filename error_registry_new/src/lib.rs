/// Custom Error type for Realis services
use backtrace::Backtrace;
use tokio::time::error::Elapsed;

// Want to Serialize and Deserialize?
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BaseError<D> {
	pub msg: String,
	// #[serde(rename = "type")]
    // pub error_type: RealisErrors,
    pub trace: String,
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
            trace: format!("{:?}", trace),
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
            data: None,
            status: None
        }
    }
}

impl<D> From<Elapsed> for BaseError<D> {
    fn from(_: Elapsed) -> Self {
        Self {
            msg: "Message reply timeout".to_string(),
            trace: "".to_string(),
            data: None,
            status: None
        }
    }
}

impl<D> From<Vec<u8>> for BaseError<D> {
    fn from(_: Vec<u8>) -> Self {
        BaseError {
            msg: "Message reply timeout".to_string(),
            trace: "".to_string(),
            data: None,
            status: None
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
