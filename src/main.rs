use dotenv::dotenv;
use hex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{thread, time::Duration};
use tokio;
use tungstenite::{connect, Message};
use url::Url;
#[tokio::main]
async fn main() {
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

    fn ethers_wei(amount: i128) -> String {
        ethers::utils::format_ether(amount).to_string()[0..4].to_string()
    }

    fn prettify_int(int: i128, decimal: i128) -> String {
        let mut s = String::new();
        let int_str = (int / 10 ^ decimal).to_string();
        let a = int_str.chars().rev().enumerate();
        for (idx, val) in a {
            if idx != 0 && idx % 3 == 0 {
                s.insert(0, ' ');
            }
            s.insert(0, val)
        }
        s
    }

    dotenv().ok();

    let (mut socket, response) =
        connect(Url::parse("ws://10.234.32.252:8546").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    let eth_block_number_json = serde_json::json!({"id": 1, "jsonrpc": "2.0", "method": "eth_blockNumber", "params": []}
    );
    let mut current_block: String = String::from("");
    let webhook = std::env::var("DISCORD_WEBHOOK").expect("We need a webhook to start");
    let client = reqwest::Client::new();
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
                    dbg!(&msg);
                    if msg.len() == 103 {
                        println!("Might be a ERC20 or ERC721");
                        let eth_call_tokenuri = serde_json::json!({"jsonrpc":"2.0","method":"eth_call",
                        "params":[{"to":get_transaction_receipt_json.result.contract_address,"data":"0xc87b56dd0000000000000000000000000000000000000000000000000000000000000000"},"latest"],"id":1});
                        socket
                            .write_message(Message::Text(eth_call_tokenuri.to_string()))
                            .unwrap();
                        msg = socket.read_message().expect("Error reading message");
                        println!("{}", msg);
                        println!("{}", msg.len());
                        if msg.len() == 80 {
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
                                },
                                {
                                    "method": "eth_call",
                                    "params": [
                                        {
                                            "data": "0x313ce567",
                                            "to": get_transaction_receipt_json.result.contract_address
                                        },
                                        "latest"
                                    ],
                                    "id": 4,
                                    "jsonrpc": "2.0"
                                }
                            ]);
                            socket
                                .write_message(Message::Text(eth_call_batch.to_string()))
                                .unwrap();
                            msg = socket.read_message().expect("Error reading message");
                            let get_eth_call_json: EthCallBundle =
                                serde_json::from_str(&msg.clone().to_string()).unwrap();

                            println!("==========================================================");
                            println!("New Token Deployed");

                            let hexstring = String::from_utf8(
                                hex::decode(
                                    get_eth_call_json[1].result.trim_start_matches("0x").clone(),
                                )
                                .unwrap(),
                            )
                            .expect("Unexpected UTF-8 Format")
                            .trim_matches(' ')
                            .to_string();
                            dbg!(&hexstring);
                            println!("Token Name : {}", hexstring);

                            println!(
                                "Contract: {}",
                                get_transaction_receipt_json.result.contract_address
                            );
                            let maxsupp = i128::from_str_radix(
                                get_eth_call_json[2].result.clone().trim_start_matches("0x"),
                                16,
                            )
                            .unwrap();
                            let decimal = i128::from_str_radix(
                                get_eth_call_json[4].result.clone().trim_start_matches("0x"),
                                16,
                            )
                            .expect("Failed to Extract Decimal");
                            let maxsupp_prettified = prettify_int(maxsupp, decimal);
                            println!("Supply: {}", maxsupp_prettified);
                            println!(
                                "Owner Address: {}",
                                get_transaction_receipt_json.result.from
                            );
                            let eth_bal = ethers_wei(
                                i128::from_str_radix(
                                    get_eth_call_json[0].result.clone().trim_start_matches("0x"),
                                    16,
                                )
                                .unwrap(),
                            );
                            println!("Owner Balance: {}ETH", eth_bal);
                            let json = json!({
                                "embeds":[{
                                    "title":"New Token Deployment",
                                    "fields": [
                                        {
                                            "name": "Name",
                                            "value" : hexstring
                                        },
                                        {
                                            "name" : "Address",
                                            "value" :  format!("https://etherscan.io/token/{}",get_transaction_receipt_json.result.contract_address)
                                        },
                                        {
                                            "name" : "Max Supply",
                                            "value" : maxsupp_prettified,
                                        },
                                        {
                                            "name" : "Owner Address",
                                            "value" : format!("https://etherscan.io/address/{}",get_transaction_receipt_json.result.from)
                                        },
                                        {
                                            "name" : "Eth Balance",
                                            "value" : format!("{}ETH",eth_bal)
                                        }
                                    ]

                                }]
                            }).to_string();
                            let response = client
                                .post(&webhook)
                                .header("Content-type", "application/json")
                                .body(json.to_owned())
                                .send()
                                .await;
                            println!("{:?}", response.expect("Cannot be"));
                            println!("==========================================================");
                        }
                    }
                }
            }
        }
    }
}
