use serde::{Deserialize, Serialize};
use serde_json::Value;
pub type EthCallBundle = Vec<EthCalls>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthCalls {
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthBlockNumberJson {
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEthBlockNumberJson {
    pub result: EthResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionReceiptJson {
    pub result: ReceiptResult,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptResult {
    pub block_hash: String,
    pub block_number: String,
    pub contract_address: String,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<Log>,
    pub logs_bloom: String,
    pub status: String,
    pub to: Value,
    pub transaction_hash: String,
    pub transaction_index: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    pub transaction_hash: String,
    pub transaction_index: String,
    pub block_hash: String,
    pub log_index: String,
    pub removed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EthResult {
    base_fee_per_gas: String,
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    gas_used: String,
    pub hash: String,
    logs_bloom: String,
    mix_hash: String,
    nonce: String,
    number: String,
    parent_hash: String,
    receipts_root: String,
    size: String,
    state_root: String,
    timestamp: String,
    total_difficulty: String,
    pub transactions: Vec<Transaction>,
    transactions_root: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    block_hash: String,
    block_number: String,
    from: String,
    gas: String,
    gas_price: String,
    max_fee_per_gas: Option<String>,
    max_priority_fee_per_gas: Option<String>,
    pub hash: String,
    input: String,
    nonce: String,
    pub to: Option<String>,
    transaction_index: String,
    value: String,
    #[serde(rename = "type")]
    type_field: String,
    #[serde(default)]
    chain_id: Option<String>,
}
