use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{Provider, Ws};
use ethers::types::{H160, U64};
use std::sync::Arc;
use std::{thread, time::Duration};
mod library;
use library::helper::*;
use library::sendwebhook::*;

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
                            .get_transaction_receipt(transaction.hash)
                            .await
                            .unwrap();
                        let address: H160 = x.unwrap().contract_address.unwrap();
                        let con_instance = ERC20::new(address, provider.clone());
                        let dummy_addy: H160 = "0x0000000000000000000000000000000000000001"
                            .parse::<Address>()
                            .unwrap();
                        let bal = con_instance.balance_of(dummy_addy).await;
                        match bal {
                            Ok(_) => {
                                let decimals = con_instance.decimals().await;
                                match decimals {
                                    Ok(decimals) => {
                                        let token_name = con_instance.name().await.unwrap();
                                        let max_supply = prettify_int(
                                            con_instance.total_supply().await.unwrap(),
                                            decimals.try_into().unwrap(),
                                        );
                                        let token_symbol = con_instance.symbol().await.unwrap();
                                        let eth_balance = ethers_wei(
                                            provider
                                                .get_balance(transaction.from, None)
                                                .await
                                                .unwrap(),
                                        );
                                        let eth_address =
                                            format!("http://etherscan.io/token/{:?}", address);
                                        let eth_owner = format!(
                                            "http://etherscan.io/address/{:?}",
                                            transaction.from
                                        );
                                        let send_hook = send_webhook(
                                            &client,
                                            token_name,
                                            max_supply,
                                            token_symbol,
                                            eth_address,
                                            eth_owner,
                                            eth_balance,
                                        )
                                        .await;
                                        match send_hook {
                                            Ok(_) => {
                                                println!("Sent webhook");
                                            }
                                            Err(e) => {
                                                println!("Error sending hook {}", e);
                                            }
                                        }
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
