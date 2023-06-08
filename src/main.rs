use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::types::{U256,U64,H160};
use serde_json::json;
use std::sync::Arc;
use std::{thread, time::Duration};
use tokio;


fn ethers_wei(amount: U256) -> String {
    ethers::utils::format_ether(amount).to_string()[0..4].to_string()
}

fn prettify_int(int: U256, decimal: i128) -> String {
    let mut s = String::new();
    let int_div_decimal = int / i128::pow(10, decimal.try_into().unwrap());
    let int_str = int_div_decimal.to_string();
    let a = int_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ' ');
        }
        s.insert(0, val)
    }
    s
}
abigen!(
    ERC20,
    r#"[
        function balanceOf(address owner) public view virtual returns (uint256)
        function decimals() public view virtual returns (uint8)
        function name() public view virtual returns (string)
        function totalSupply() public view virtual returns (uint256)
        function symbol() public view virtual returns (string)
    ]"#,
);

#[tokio::main]
async fn main() {

    dotenv().ok();
    let rpc_url = std::env::var("RPC_URL").expect("We need a WS to start");
    let provider = Provider::<Ws>::connect(&rpc_url)
        .await
        .expect("Error Connecting to WS");
    let provider = Arc::new(provider);
    let mut current_block: String = String::from("");
    let webhook = std::env::var("DISCORD_WEBHOOK").expect("We need a webhook to start");
    let client = reqwest::Client::new();
    loop {
        let block_number: U64 = provider.get_block_number().await.unwrap();
        if current_block == block_number.to_string() {
            thread::sleep(Duration::from_secs(8));
        } else {
            println!("Block Number: {:?}", block_number);
            current_block = block_number.clone().to_string();
            let eth_get_block_number = provider.get_block_with_txs(block_number).await.unwrap();
            match eth_get_block_number {
                Some(_) => {}
                None => {
                    println!("Couldn't get Transactions on this Block");
                    panic!();
                }
            }
            for transaction in eth_get_block_number.unwrap().transactions.iter() {
                match transaction.to {
                    Some(_) => {}
                    None => {
                        println!("Contract creation at TX {:?}", transaction.hash);
                        let x = provider
                            .get_transaction_receipt(transaction.hash.clone())
                            .await
                            .unwrap();
                        let address: H160 = x.unwrap().contract_address.unwrap();
                        let con_instance = ERC20::new(address.clone(), provider.clone());
                        let dummy_addy: H160 = "0x0000000000000000000000000000000000000001"
                            .parse::<Address>()
                            .unwrap();
                        let bal = con_instance.balance_of(dummy_addy).await;
                        match bal {
                            Ok(_) => {
                                let decimals = con_instance.decimals().await;
                                match decimals {
                                    Ok(decimals) => {
                                        let token_name = con_instance.name().await;
                                        let total_supply = con_instance.total_supply().await;
                                        let token_symbol = con_instance.symbol().await;
                                        let eth_balance = provider.get_balance(transaction.from, None).await.unwrap();
                                        let  eth_address = format!("http://etherscan.io/token/{:?}",address);
                                        let  eth_owner = format!("http://etherscan.io/address/{:?}",transaction.from);
                                        let json = json!({
                                                                "embeds":[{
                                                                    "title":"New Token Deployment",
                                                                    "fields": [
                                                                        {
                                                                            "name": "Name",
                                                                            "value" : format!("{}({})",token_name.unwrap(),token_symbol.unwrap()),
                                                                        },
                                                                        {
                                                                            "name" : "Address",
                                                                            "value" : eth_address,
                                                                        },
                                                                        {
                                                                            "name" : "Max Supply",
                                                                            "value" : prettify_int(total_supply.unwrap(), decimals.try_into().unwrap()),
                                                                        },
                                                                        {
                                                                            "name" : "Owner Address",
                                                                            "value" : eth_owner,
                                                                        },
                                                                        {
                                                                            "name" : "Eth Balance",
                                                                            "value" : format!("{}ETH",ethers_wei(eth_balance))
                                                                        }
                                                                    ]
                                                            
                                
                                                                }]
                                                            }).to_string();
                                        println!("{:?}",json);
                                        let response = client
                                        .post(&webhook)
                                        .header("Content-type", "application/json")
                                        .body(json.to_owned())
                                        .send()
                                        .await;
                                        println!("{:?}", response.expect("Cannot be"));
                                        println!("==========================================================");
                                    }
                                    Err(_) => {
                                        println!("Not an ERC20")
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Most likely not an ERC20 or ERC721 but here is the result when calling balanceOf : {:?} ", e);
                            }
                        }
                    }
                }
            }

        }
    }
}
