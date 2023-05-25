use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::i64;
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
    struct GetEthBlockNumberJson {
        jsonrpc: String,
        id: i64,
        result: Result,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct GetTransactionReceiptJson {
        jsonrpc: String,
        id: i64,
        result: ReceiptResult,
    }
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReceiptResult {
        block_hash: String,
        block_number: String,
        contract_address: String,
        cumulative_gas_used: String,
        effective_gas_price: String,
        from: String,
        gas_used: String,
        logs: Vec<Log>,
        logs_bloom: String,
        status: String,
        to: Value,
        transaction_hash: String,
        transaction_index: String,
        #[serde(rename = "type")]
        type_field: String,
    }
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Log {
        address: String,
        topics: Vec<String>,
        data: String,
        block_number: String,
        transaction_hash: String,
        transaction_index: String,
        block_hash: String,
        log_index: String,
        removed: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Result {
        base_fee_per_gas: String,
        difficulty: String,
        extra_data: String,
        gas_limit: String,
        gas_used: String,
        hash: String,
        logs_bloom: String,
        miner: String,
        mix_hash: String,
        nonce: String,
        number: String,
        parent_hash: String,
        receipts_root: String,
        #[serde(rename = "sha3Uncles")]
        sha3uncles: String,
        size: String,
        state_root: String,
        timestamp: String,
        total_difficulty: String,
        transactions: Vec<Transaction>,
        transactions_root: String,
        uncles: Vec<Value>,
        withdrawals: Vec<Withdrawal>,
        withdrawals_root: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Transaction {
        block_hash: String,
        block_number: String,
        from: String,
        gas: String,
        gas_price: String,
        max_fee_per_gas: Option<String>,
        max_priority_fee_per_gas: Option<String>,
        hash: String,
        input: String,
        nonce: String,
        to: Option<String>,
        transaction_index: String,
        value: String,
        #[serde(rename = "type")]
        type_field: String,
        #[serde(default)]
        access_list: Vec<AccessList>,
        chain_id: Option<String>,
        v: String,
        r: String,
        s: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AccessList {
        address: String,
        storage_keys: Vec<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Withdrawal {
        index: String,
        validator_index: String,
        address: String,
        amount: String,
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
            println!(
                "Block Number: {}",
                i64::from_str_radix(
                    block_response_json.result.clone().trim_start_matches("0x"),
                    16
                )
                .unwrap()
            );
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
                    let get_transaction_receipt_json: GetTransactionReceiptJson =
                        serde_json::from_str(&msg.clone().to_string()).unwrap();

                    let eth_call_json = serde_json::json!({"jsonrpc":"2.0","method":"eth_call",
                    "params":[{"to":get_transaction_receipt_json.result.contract_address,"data":"0x70a082310000000000000000000000000000000000000000000000000000000000000001"},"latest"],"id":1});
                    println!("{}", eth_call_json);
                    socket
                        .write_message(Message::Text(eth_call_json.to_string()))
                        .unwrap();
                    msg = socket.read_message().expect("Error reading message");
                    println!(
                        "New Smart Contract Deployment at TX Hash: {} with the contract Address of {} deployer is {}",
                        transaction.hash,get_transaction_receipt_json.result.contract_address,get_transaction_receipt_json.result.from
                    );
                    println!("{}", msg);
                }
            }
        }
    }
}
