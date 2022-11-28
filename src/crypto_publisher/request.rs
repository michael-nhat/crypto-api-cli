

#[tokio::main]
async fn get_coin_price(request_type: String, request_currency: String, request_rates: String) -> Result<std::string::String, Box<dyn std::error::Error>> {

    let request_url = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/{type}",
        currency = request_currency,
        rates = request_rates,
        type = request_type);

    let client = Client::new();
    let resp_price = client.get(&request_url).send().await?.json::<CoinbasePrice>().await?;
 
    Ok(resp_price.data.amount)
}

#[tokio::main]
async fn get_coin_time() -> Result<std::string::String, Box<dyn std::error::Error>> {    

    let request_url = format!("https://api.coinbase.com/v2/time");

    let client = Client::new();
    let resp_time = client.get(&request_url).send().await?.json::<CoinbaseTime>().await?;
 
    Ok(resp_time.data.iso)
 }