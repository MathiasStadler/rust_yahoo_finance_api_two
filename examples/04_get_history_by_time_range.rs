// FROM HERE
// https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/
use yahoo_finance_api as yahoo;
use tokio_test;
// extern crate chrono;
use chrono::{ TimeZone, Utc};
// use rust_decimal::prelude::*;
// use chrono::prelude::*;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range("AAPL", "1d", "1mo")).unwrap();
    let quotes = response.quotes().unwrap();
    // println!("Apple's quotes of the last month: {:?}", quotes);
    for line in quotes.iter() {
        let date_time = Utc.timestamp_opt(line.timestamp as i64, 0).unwrap();
        let open = line.open;
        let high = line.high;
        let low = line.low;
        let close = line.close;
        let volume = line.volume;
        // print_type_of(&open);
        // println!("open {:.2}", &open);
        // let open = Decimal::from_str(line.open).unwrap();
        //OHLCV
        println!("{},{:.2},{:.2},{:.2},{:.2},{:.2}",date_time,open,high,low,close,volume);
    }
}
