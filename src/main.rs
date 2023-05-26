use hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{thread, time::Duration};
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    type EthCallBundle = Vec<EthCalls>;
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EthCalls {
        pub result: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct EthBlockNumberJson {
        result: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct GetEthBlockNumberJson {
        result: Result,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct GetTransactionReceiptJson {
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
        mix_hash: String,
        nonce: String,
        number: String,
        parent_hash: String,
        receipts_root: String,
        size: String,
        state_root: String,
        timestamp: String,
        total_difficulty: String,
        transactions: Vec<Transaction>,
        transactions_root: String,
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
        chain_id: Option<String>,
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
                i128::from_str_radix(
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

                    let eth_call_balance_of = serde_json::json!({"jsonrpc":"2.0","method":"eth_call",
                    "params":[{"to":get_transaction_receipt_json.result.contract_address,"data":"0x70a082310000000000000000000000000000000000000000000000000000000000000001"},"latest"],"id":1});
                    socket
                        .write_message(Message::Text(eth_call_balance_of.to_string()))
                        .unwrap();
                    msg = socket.read_message().expect("Error reading message");
                    //let placeholder = String::from("Placeholder");
                    println!("{}", msg);
                    if msg.len() == 103 {
                        let eth_call_tokenuri = serde_json::json!({"jsonrpc":"2.0","method":"eth_call",
                        "params":[{"to":get_transaction_receipt_json.result.contract_address,"data":"0xc87b56dd0000000000000000000000000000000000000000000000000000000000000001"},"latest"],"id":1});
                        socket
                            .write_message(Message::Text(eth_call_tokenuri.to_string()))
                            .unwrap();
                        msg = socket.read_message().expect("Error reading message");
                        println!("{}", msg);
                        println!("{}", msg.len());

                        let eth_call_batch = serde_json::json!([
                            {
                                "method": "eth_getBalance",
                                "params": [
                                    get_transaction_receipt_json.result.from,
                                    "latest"
                                ],
                                "id": 3,
                                "jsonrpc": "2.0"
                            },
                            {
                                "method": "eth_call",
                                "params": [
                                    {
                                        "data": "0x06fdde03",
                                        "to": get_transaction_receipt_json.result.contract_address
                                    },
                                    "latest"
                                ],
                                "id": 1,
                                "jsonrpc": "2.0"
                            },
                            {
                                "method": "eth_call",
                                "params": [
                                    {
                                        "data": "0x18160ddd",
                                        "to": get_transaction_receipt_json.result.contract_address
                                    },
                                    "latest"
                                ],
                                "id": 2,
                                "jsonrpc": "2.0"
                            }
                        ]);
                        socket
                            .write_message(Message::Text(eth_call_batch.to_string()))
                            .unwrap();
                        msg = socket.read_message().expect("Error reading message");
                        println!("{}", msg);
                        let get_eth_call_json: EthCallBundle =
                            serde_json::from_str(&msg.clone().to_string()).unwrap();
                        //dbg!(get_eth_call_json);

                        println!("==========================================================");
                        println!("New Token Deployed");

                        let hexstring = hex::decode(
                            get_eth_call_json[1].result.trim_start_matches("0x").clone(),
                        )
                        .unwrap();

                        println!("Token Name : {}", String::from_utf8(hexstring).unwrap());

                        println!(
                            "Contract: {}",
                            get_transaction_receipt_json.result.contract_address
                        );
                        println!(
                            "Supply: {}",
                            i128::from_str_radix(
                                get_eth_call_json[2].result.clone().trim_start_matches("0x"),
                                16
                            )
                            .unwrap()
                        );
                        println!(
                            "Owner Address: {}",
                            get_transaction_receipt_json.result.from
                        );
                        println!(
                            "Owner Balance: {}",
                            (i128::from_str_radix(
                                get_eth_call_json[0].result.clone().trim_start_matches("0x"),
                                16
                            )
                            .unwrap())
                                / 10
                                ^ 18
                        );
                        println!("==========================================================");
                    }
                }
            }
        }
    }
}
