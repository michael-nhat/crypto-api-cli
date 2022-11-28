
use serde::{Serialize, Deserialize};
#[derive(Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice
}

#[derive(Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinbaseTime {
    pub data: CoinTime
}

#[derive(Deserialize, Debug)]
pub struct CoinTime {
    pub iso: String,
    pub epoch: i64,    
}

#[derive(Serialize, Debug)]
pub struct CryptoPriceData {
    pub data: CryptoPrice
}

#[derive(Serialize, Debug)]
pub struct CryptoPrice {
    pub quote_time: String,
    pub currency: String,
    pub rate: String,
    pub spot_price: String,
    pub buy_price: String,
    pub sell_price: String,
    pub spread_price: String,
}
