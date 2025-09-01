use futures::{ future, StreamExt };
use yahoo_finance::Streamer;

#[tokio::main]
async fn main() {
   let mut streamer = Streamer::new(vec!["AAPL", "^DJI", "^IXIC"]);

   streamer.stream().await
      .for_each(|quote| {
         println!("At {}, {} is trading for ${} [{}]", quote.timestamp, quote.symbol, quote.price, quote.volume);

         future::ready(())
      })
      .await;
}