// FROM HERE
// https://docs.rs/yahoo_finance_api/latest/yahoo_finance_api/
// use chrono::{TimeZone, Utc};
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    // let response = tokio_test::block_on(provider.get_quote_range("TREX", "1d", "1mo")).unwrap();
    let symbol = "TREX";
    let response = tokio_test::block_on(provider.get_ticker_info(symbol));
    // let quotes = response.quotes().unwrap();
    // for line in quotes.iter() {
    //     let date_time = Utc.timestamp_opt(line.timestamp as i64, 0).unwrap();
    //     let open = line.open;
    //     let high = line.high;
    //     let low = line.low;
    //     let close = line.close;
    //     let volume = line.volume;

    //     //OHLCV
    //     println!(
    //         "{},{:.2},{:.2},{:.2},{:.2},{:.2}", 
    //         date_time, open, high, low, close, volume
    //     );
    // }
}
