use crate::book::OrderBook;
use crate::order::{Order, Side, Trade};

/// Repeatedly match a taker until it is filled or no resting order crosses;
/// the remainder (if any) rests on the book. Returns all trades produced.
pub fn match_order(book: &mut OrderBook, mut taker: Order) -> Vec<Trade> {
    let mut trades = Vec::new();
    while taker.quantity > 0 {
        match try_one(book, &mut taker) {
            Some(t) => trades.push(t),
            None => break,
        }
    }
    if taker.quantity > 0 {
        book.add_resting(taker);
    }
    trades
}

fn try_one(book: &mut OrderBook, taker: &mut Order) -> Option<Trade> {
    let level = match taker.side {
        Side::Buy => &mut book.asks,
        Side::Sell => &mut book.bids,
    };
    let best_idx = match taker.side {
        Side::Buy => level.iter().enumerate().min_by_key(|(_, o)| o.price),
        Side::Sell => level.iter().enumerate().max_by_key(|(_, o)| o.price),
    }.map(|(i, _)| i)?;

    let maker = &mut level[best_idx];
    let crosses = match taker.side {
        Side::Buy => maker.price <= taker.price,
        Side::Sell => maker.price >= taker.price,
    };
    if !crosses { return None; }

    let qty = taker.quantity.min(maker.quantity);
    let trade = Trade { taker: taker.id, maker: maker.id, price: maker.price, quantity: qty };
    taker.quantity -= qty;
    maker.quantity -= qty;
    if maker.quantity == 0 { level.remove(best_idx); }
    Some(trade)
}
