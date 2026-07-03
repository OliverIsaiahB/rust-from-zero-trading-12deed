mod order;
mod book;
mod engine;
mod tests;

use order::{Order, Side};
use book::OrderBook;
use engine::match_order;

fn main() {
    let mut book = OrderBook::default();
    book.add_resting(Order::new(1, Side::Sell, 10_150, 3));
    book.add_resting(Order::new(2, Side::Sell, 10_160, 5));

    // A buyer for 6 @ 10_160 sweeps the 3 @ 10_150 then 3 of the 5 @ 10_160.
    let trades = match_order(&mut book, Order::new(3, Side::Buy, 10_160, 6));
    for t in &trades {
        println!("TRADE {}x @ {} (taker {} / maker {})", t.quantity, t.price, t.taker, t.maker);
    }
    println!("best ask now: {:?}", book.best_ask());
}
