#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
}

impl Order {
    pub fn new(id: u64, side: Side, price: u64, quantity: u64) -> Order {
        Order { id, side, price, quantity }
    }
    pub fn notional(&self) -> u64 {
        self.price * self.quantity
    }
    pub fn describe(&self) -> &'static str {
        match self.side { Side::Buy => "buy", Side::Sell => "sell" }
    }
}

/// The record produced when two orders trade.
#[derive(Debug, PartialEq)]
pub struct Trade {
    pub taker: u64,
    pub maker: u64,
    pub price: u64,
    pub quantity: u64,
}
