extern crate serde;

extern crate reqwest;

extern crate clap;

use clap::Parser;

use log::info;
use std::{thread::sleep, time::Duration};

use rdkafka::util::get_rdkafka_version;

// use crate::event::publish2;

mod publish;
use publish::publish;
mod coin_struct;
use coin_struct::{ CryptoPrice, CryptoPriceData};

mod request;
mod cli;
use cli::Cli;
pub use request::{get_coin_price, get_coin_time};

pub fn crypto_publisher() {
    let args = Cli::parse();
    let mut count = 0i32;

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    loop {
        if count == args.frequency {
            break;
        }

        let currency = &args.currency;
        let rates = &args.rates;
        let quote_time = get_coin_time();
        let broker = &args.broker;
        let topic = &args.topic;

        let spot_price =
            get_coin_price("spot".to_string(), currency.to_string(), rates.to_string());
        let buy_price = get_coin_price("buy".to_string(), currency.to_string(), rates.to_string());
        let sell_price =
            get_coin_price("sell".to_string(), currency.to_string(), rates.to_string());

        let quote_time = quote_time.as_ref();
        let spot_price = spot_price.as_ref();
        let buy_price = buy_price.as_ref();
        let sell_price = sell_price.as_ref();

        let spread_price: f32 = (buy_price.unwrap().parse::<f32>().unwrap())
            - (&sell_price.unwrap().parse::<f32>().unwrap());

        let price_screen = format!(
            "{}: {}-{} SPOT Price: {} | BUY Price: {} | SELL Price: {} | Price Spread: {}",
            quote_time.unwrap(),
            currency.to_string(),
            rates.to_string(),
            spot_price.unwrap(),
            buy_price.unwrap(),
            sell_price.unwrap(),
            spread_price.to_string()
        );

        println!("{}", price_screen);

        let price_struct = CryptoPriceData {
            data: CryptoPrice {
                quote_time: quote_time.unwrap().to_string(),
                currency: currency.to_string(),
                rate: rates.to_string(),
                spot_price: spot_price.unwrap().to_string(),
                buy_price: buy_price.unwrap().to_string(),
                sell_price: sell_price.unwrap().to_string(),
                spread_price: spread_price.to_string(),
            },
        };

        let price_json = serde_json::to_string(&price_struct).unwrap();
        publish(broker, topic, &price_json, count);

        sleep(Duration::from_millis(args.interval));
        count += 1;
    }
}
