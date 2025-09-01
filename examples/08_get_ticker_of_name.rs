// FOUND HERE
//  Search for a ticker given a search string (e.g. company name)
// https://github.com/xemwebe/yahoo_finance_api
use yahoo_finance_api as yahoo;
use tokio_test;

fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let resp = tokio_test::block_on(provider.search_ticker("Apple")).unwrap();

    // let mut apple_found = false;
    println!("All tickers found while searching for 'Apple':");
    for item in resp.quotes
    {
        println!("{}", item.symbol)
    }
}