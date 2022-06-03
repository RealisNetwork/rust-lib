use crate::requests::{Response, ResponseMessage, ResponseResult};
use error_registry::BaseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainResponseParams {
    #[serde(rename = "blockId")]
    pub block_id: String,
    #[serde(rename = "txId")]
    pub hash_id: String,
}

impl BlockchainResponseParams {
    pub fn into_response<T>(self, request: T) -> Response<T, BlockchainResponseParams, ()> {
        Response {
            result: ResponseResult {
                request,
                response: ResponseMessage::Right { value: self },
            },
        }
    }

    pub fn into_left_response<T>(request: T, msg: String, error_type: BaseError<()>) -> Response<T, (), ()> {
        Response {
            result: ResponseResult {
                request,
                response: ResponseMessage::Left {
                    value: BaseError::<()>::new(msg, error_type.error_type, None, None),
                },
            },
        }
    }
}
