use yahoo_finance_api as yahoo;
use time::OffsetDateTime;
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("AAPL", "1d")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    let time: OffsetDateTime =
        OffsetDateTime::from_unix_timestamp(quote.timestamp).unwrap();
    println!("At {} quote price of Apple was {}", time, quote.close);
}

