// FOUND HERE
//  Search for a ticker given a search string (e.g. company name)
// https://github.com/xemwebe/yahoo_finance_api
use yahoo_finance_api as yahoo;
use tokio_test;
use yahoo_finance_api::YQuoteItem;
fn main() {
   let  provider = yahoo::YahooConnector::new().unwrap();
   
   let mut ticker = tokio_test::block_on(provider.search_ticker("Apple")).unwrap();

   print!("{:?}",ticker.count);
   println!("{:?}",ticker.quotes);

    for i in ticker.quotes.iter_mut() {
        //println!("{:?}",i);
        let a: &mut YQuoteItem = i;
        
        println!("exchange => {}",a.exchange);
        println!("short_name => {}",a.short_name);
        println!("long_name => {}",a.long_name);
        println!("symbol => {}",a.symbol);
        println!("NEXT NEXT");
  }

    //get_ticker_info
    //let resp = tokio_test::block_on(provider.get_ticker_info("TREX")).unwrap();

    
    // resp.quote_summary
    
    // resp.finance.
    // // let mut apple_found = false;
    // println!("All tickers found while searching for 'Apple':");
    // for item in resp.quotes
    // {
    //     println!("{}", item.symbol)
    // }
}