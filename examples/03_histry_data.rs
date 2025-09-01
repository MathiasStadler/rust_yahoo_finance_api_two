use yahoo_finance_api as yahoo;
#[allow(unused_imports)]
use time::{macros::datetime, OffsetDateTime};
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    // returns historic quotes with daily interval
    let resp = tokio_test::block_on(provider.get_quote_history("AAPL", start, end)).unwrap();
    let quotes = resp.quotes().unwrap();
    println!("Apple's quotes in January: {:?}", quotes);
}