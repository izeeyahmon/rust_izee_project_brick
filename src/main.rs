use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{thread, time::Duration};
use tungstenite::{connect, Message};
use url::Url;
fn main() {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct EthBlockNumberJson {
        jsonrpc: String,
        id: u32,
        result: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetEthBlockNumberJson {
        pub jsonrpc: String,
        pub id: i64,
        pub result: Result,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Result {
        pub base_fee_per_gas: String,
        pub difficulty: String,
        pub extra_data: String,
        pub gas_limit: String,
        pub gas_used: String,
        pub hash: String,
        pub logs_bloom: String,
        pub miner: String,
        pub mix_hash: String,
        pub nonce: String,
        pub number: String,
        pub parent_hash: String,
        pub receipts_root: String,
        #[serde(rename = "sha3Uncles")]
        pub sha3uncles: String,
        pub size: String,
        pub state_root: String,
        pub timestamp: String,
        pub total_difficulty: String,
        pub transactions: Vec<Transaction>,
        pub transactions_root: String,
        pub uncles: Vec<Value>,
        pub withdrawals: Vec<Withdrawal>,
        pub withdrawals_root: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        pub block_hash: String,
        pub block_number: String,
        pub from: String,
        pub gas: String,
        pub gas_price: String,
        pub max_fee_per_gas: Option<String>,
        pub max_priority_fee_per_gas: Option<String>,
        pub hash: String,
        pub input: String,
        pub nonce: String,
        pub to: Option<String>,
        pub transaction_index: String,
        pub value: String,
        #[serde(rename = "type")]
        pub type_field: String,
        #[serde(default)]
        pub access_list: Vec<AccessList>,
        pub chain_id: Option<String>,
        pub v: String,
        pub r: String,
        pub s: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AccessList {
        pub address: String,
        pub storage_keys: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Withdrawal {
        pub index: String,
        pub validator_index: String,
        pub address: String,
        pub amount: String,
    }

    let (mut socket, response) =
        connect(Url::parse("ws://10.234.32.252:8546").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    let eth_block_number_json = serde_json::json!({"id": 1, "jsonrpc": "2.0", "method": "eth_blockNumber", "params": []}
    );
    let mut current_block: String = String::from("");
    loop {
        socket
            .write_message(Message::Text(eth_block_number_json.to_string()))
            .unwrap();
        let mut msg = socket.read_message().expect("Error reading message");
        let block_response_json: EthBlockNumberJson =
            serde_json::from_str(&msg.clone().to_string()).unwrap();

        if current_block == block_response_json.result {
            thread::sleep(Duration::from_secs(8));
        } else {
            println!("Received: {}", block_response_json.result);
            current_block = block_response_json.result.clone();
            let eth_get_block_number_json = serde_json::json!({"id": 1, "jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": [&block_response_json.result,true]});
            socket
                .write_message(Message::Text(eth_get_block_number_json.to_string()))
                .unwrap();
            msg = socket.read_message().expect("Error reading message");
            let get_block_response_json: GetEthBlockNumberJson =
                serde_json::from_str(&msg.clone().to_string()).unwrap();

            for transaction in get_block_response_json.result.transactions.iter() {
                if transaction.to.is_none() {
                    let eth_get_transactionreceipt_json = serde_json::json!({"jsonrpc":"2.0","method":"eth_getTransactionReceipt","params":[transaction.hash],"id":1});
                    socket
                        .write_message(Message::Text(eth_get_transactionreceipt_json.to_string()))
                        .unwrap();
                    msg = socket.read_message().expect("Error reading message");
                    println!(
                        "New Smart Contract Deployment at TX Hash: {}",
                        transaction.hash
                    );
                    println!("{}", msg);
                }
            }
        }
    }
}
