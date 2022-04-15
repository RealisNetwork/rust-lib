use serde::{Deserialize, Serialize};
use error_registry::RealisErrors;
use crate::requests::{Response, ResponseError, ResponseMessage, ResponseResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainResponseParams {
    #[serde(rename = "blockId")]
    pub block_id: String,
    #[serde(rename = "txId")]
    pub hash_id: String,
}

impl BlockchainResponseParams {
    pub fn into_response<T>(self, request: T) -> Response<T, BlockchainResponseParams> {
        Response {
            result: ResponseResult {
                request,
                response: ResponseMessage::Right {
                    value: self
                }
            }
        }
    }

    pub fn into_left_response<T>(request: T, msg: String, error_type: RealisErrors) -> Response<T, ()> {
        Response {
            result: ResponseResult {
                request,
                response: ResponseMessage::Left {
                    value: ResponseError {
                        msg,
                        error_type,
                        ..Default::default()
                    },
                }
            }
        }
    }
}
