use crate::order::{Order, Side};

/// The order book: resting buyers (bids) and sellers (asks).
#[derive(Default)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

impl OrderBook {
    /// Add an order that did not match — it rests on its side of the book.
    pub fn add_resting(&mut self, order: Order) {
        match order.side {
            Side::Buy => self.bids.push(order),
            Side::Sell => self.asks.push(order),
        }
    }

    /// Best bid = highest price a buyer will pay.
    pub fn best_bid(&self) -> Option<u64> {
        self.bids.iter().map(|o| o.price).max()
    }

    /// Best ask = lowest price a seller will accept.
    pub fn best_ask(&self) -> Option<u64> {
        self.asks.iter().map(|o| o.price).min()
    }
}
