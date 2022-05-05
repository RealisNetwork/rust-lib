pub const CREATE_BINANCE_WALLET_TOPIC_TO_RESPONSE: &'static str = "create_binance_wallet_topic_to_response";

// - Orchestrator - adapter topics
pub const ORCHESTRATOR_ADD_NFT_ITEM_TOPIC: &'static str = "add_nft_item";
pub const ORCHESTRATOR_REMOVE_NFT_ITEM_TOPIC: &'static str = "remove_nft_item";
pub const ORCHESTRATOR_INCREASE_BALANCE_TOPIC: &'static str = "debit_hard_currency";
pub const ORCHESTRATOR_DECREASE_BALANCE_TOPIC: &'static str = "credit_hard_currency";
pub const ORCHESTRATOR_TRANSFER_TOKEN_FROM_PLAYER_TOPIC: &'static str = "transfer_token_from_player";
pub const ORCHESTRATOR_TRANSFER_NFT_FROM_PLAYER_TOPIC: &'static str = "transfer_nft_from_player";
// - Orchestrator - external transfer topics
pub const ORCHESTRATOR_EXTERNAL_TRANSFER_NFT_TOPIC: &'static str = "transfer_nft";
pub const ORCHESTRATOR_EXTERNAL_TRANSFER_BALANCE_TOPIC: &'static str = "transfer_balance";
// - Orchestrator - marketplace topics
pub const ORCHESTRATOR_CHANGE_PRICE_NFT_TOPIC: &'static str = "change_price_nft";
pub const ORCHESTRATOR_BUY_NFT_TOPIC: &'static str = "buy_nft";
pub const ORCHESTRATOR_REMOVE_NFT_TOPIC: &'static str = "remove_nft";
pub const ORCHESTRATOR_SELL_NFT_TOPIC: &'static str = "sell_nft";
// - Orchestrator requests
pub const ORCHESTRATOR_ADD_EXTERNAL_REALIS_WALLET_TOPIC: &'static str = "add_external_realis_wallet";
pub const ORCHESTRATOR_GET_ACCOUNT_ID_BY_USER_ID_TOPIC: &'static str = "get_account_id_by_user_id";
pub const ORCHESTRATOR_GET_EXTERNAL_REALIS_WALLET_TOPIC: &'static str = "get_external_realis_wallet";
pub const ORCHESTRATOR_REMOVE_EXTERNAL_REALIS_WALLET_TOPIC: &'static str = "remove_external_realis_wallet";
pub const ORCHESTRATOR_GET_USER_BY_BINANCE_TOPIC: &'static str = "get_user_by_binance";
pub const ORCHESTRATOR_GET_USER_BY_REALIS_TOPIC: &'static str = "get_user_by_realis";
// - Orchestrator response_message
pub const ORCHESTRATOR_ADAPTER_RESPONSE_TOPIC: &'static str = "adapter_response";
pub const ORCHESTRATOR_NFT_MINT_NOTIFICATION_TOPIC: &'static str = "nft_notification";
pub const ORCHESTRATOR_BALANCE_INCREASE_BALANCE_BY_USER_ID_TOPIC: &'static str = "balance_increaseBalanceByUserId";
// - Orchestrator - storage request
pub const ORCHESTRATOR_GET_BALANCE_TOPIC: &'static str = "get_balance";
pub const ORCHESTRATOR_GET_NFT_LIST_TOPIC: &'static str = "get_nft_list";
pub const ORCHESTRATOR_GET_NFT_LIST_WITH_OFFSET_TOPIC: &'static str = "get_nft_list_with_offset";
// - Orchestrator - withdraw request
pub const ORCHESTRATOR_WITHDRAW_NFT_BINANCE_TOPIC: &'static str = "withdraw_nft";
pub const ORCHESTRATOR_WITHDRAW_NFT_REALIS_TOPIC: &'static str = "withdraw_nft";
pub const ORCHESTRATOR_BINANCE_WITHDRAW_TOKENS_TOPIC: &'static str = "withdraw_tokens";
pub const ORCHESTRATOR_REALIS_WITHDRAW_TOKENS_TOPIC: &'static str = "withdraw_tokens";
// - Orchestrator other topics
pub const ORCHESTRATOR_EXTRINSIC_CONFIRMATION_TOPIC: &'static str = "extrinsic_confirmation";
// - Adapter topics
pub const ADAPTER_ADD_NFT_ITEM_TOPIC: &'static str = "adapter-add_nft_item";
pub const ADAPTER_REMOVE_NFT_ITEM_TOPIC: &'static str = "adapter-remove_nft_item";
pub const ADAPTER_INCREASE_BALANCE_TOPIC: &'static str = "adapter-increase_balance";
pub const ADAPTER_DECREASE_BALANCE_TOPIC: &'static str = "adapter-decrease_balance";
pub const ADAPTER_TRANSFER_TOKEN_FROM_PLAYER_TOPIC: &'static str = "adapter-transfer_token_from_player";
pub const ADAPTER_TRANSFER_NFT_FROM_PLAYER_TOPIC: &'static str = "adapter-transfer_nft_from_player";
pub const ADAPTER_CHANGE_BATCH_TOPIC: &'static str = "adapter-change_batch";
// - Storage topics
pub const STORAGE_GET_BALANCE_TOPIC: &'static str = "storage-get_balance";
pub const STORAGE_GET_MARKETPLACE_NFT_LIST_TOPIC: &'static str = "storage-get_marketplace_nft_list";
pub const STORAGE_GET_NFT_LIST_TOPIC: &'static str = "storage-get_nft_list";
pub const STORAGE_GET_NFT_LIST_WITH_OFFSET_TOPIC: &'static str = "storage-get_nft_list_with_offset";
