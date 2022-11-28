extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author,
    version = "0.1.0",
    about = "Crypto Price Publisher - Command Line Interface (CLI) Application"
)]
pub struct Cli {
    /// Currency Symbol. An example would be BTC.
    #[clap(short, long, default_value = "BTC")]
    pub currency: String,
    #[clap(short, long, default_value = "USD")]
    /// Rates Symbol. An example would be USD.
    pub rates: String,
    #[clap(short, long, default_value = "30000")]
    /// Interval. An example would be an interval of 30000 miliseconds or 30 seconds.
    pub interval: u64,
    #[clap(short, long, default_value = "10")]
    /// Repeat Frequency. An example would be repeat frequency of 10 times.
    pub frequency: i32,
    #[clap(short, long, default_value = "localhost:9092")]
    /// Broker.
    pub broker: String,
    #[clap(short, long, default_value = "crypto_price")]
    /// Topic Name.
    pub topic: String,
}
