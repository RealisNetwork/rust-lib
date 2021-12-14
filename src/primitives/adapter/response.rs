use crate::primitives::adapter::{
    types::{Amount, BlockNumber, Hash, UserId},
    Request,
};
use realis_primitives::TokenId;

use serde_json::json;

pub enum Response {
    IncreaseBalance(UserId, Amount, Hash, BlockNumber),

    TransferNftToGameAccount(UserId, TokenId, Hash, BlockNumber),

    TransferNftToConnectedAccount(UserId, TokenId, Hash, BlockNumber),
    TransferNftFromConnectedAccount(UserId, TokenId, Hash, BlockNumber),

    NftMinted(Request, Hash, BlockNumber),
    NftMintedError(Request, Hash, BlockNumber),
}

impl Response {
    #[must_use]
    pub fn to_payload(&self) -> (String, String) {
        match self {
            Response::NftMinted(request, tx_hash, _) => {
                let subject = request.topic_res.clone();
                (
                    json!({
                        "result": {
                            "request": request,
                            "response": {
                                "type": "Right",
                                "value": {
                                    "txId": tx_hash
                                }
                            }
                        }
                    })
                    .to_string(),
                    subject,
                )
            }
            Response::IncreaseBalance(user_id, amount, hash, _) => {
                (
                    json!({
                        "id": uuid::Uuid::new_v4(),
                        "agent": "transactions",
                        "method": "balance_increaseBalanceByUserId",
                        "lang": "en",
                        "params": {
                          "extraDetails": {},
                          "creator": "realis-adapter",
                          "reason": "onChain.deposit", // TODO ask Paul maybe credit?
                          "currency": "LIS",
                          "amount": amount.to_string(),
                          "txId": hash,
                          "userId": user_id,
                        }
                    })
                    .to_string(),
                    String::from("balance_increaseBalanceByUserId"),
                )
            }
            Response::TransferNftToGameAccount(_, _, _, _) => (String::from(""), String::from("")),
            Response::TransferNftToConnectedAccount(_, _, _, _)
            | Response::TransferNftFromConnectedAccount(_, _, _, _) => {
                (String::from(""), String::from(""))
            }
            Response::NftMintedError(request, _, _) => {
                let subject = request.topic_res.clone();
                (
                    json!({
                        "result": {
                            "request": request,
                            "response": {
                                "type": "Left",
                                "value": {
                                    "msg": "Cannot mint nft!"
                                }
                            }
                        }
                    })
                    .to_string(),
                    subject,
                )
            }
        }
    }
}
