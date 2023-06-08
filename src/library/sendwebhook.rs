use ::reqwest::Client;
use serde_json::json;
pub async fn send_webhook(
    client: &Client,
    token_name: String,
    total_supply: String,
    token_symbol: String,
    address: String,
    owner: String,
    balance: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let webhook = std::env::var("DISCORD_WEBHOOK").expect("We need a webhook to start");
    let json = json!({
        "embeds":[{
            "title":"New Token Deployment",
            "fields": [
                {
                    "name": "Name",
                    "value" : format!("{}({})",token_name,token_symbol),
                },
                {
                    "name" : "Address",
                    "value" : address,
                },
                {
                    "name" : "Total Supply",
                    "value" : total_supply,
                },
                {
                    "name" : "Owner Address",
                    "value" : owner,
                },
                {
                    "name" : "Eth Balance",
                    "value" : format!("{}ETH",balance)
                }
            ]


        }]
    })
    .to_string();
    let _response = client
        .post(&webhook)
        .header("Content-type", "application/json")
        .body(json.to_owned())
        .send()
        .await;
    Ok(())
}
