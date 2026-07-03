#[cfg(test)]
mod tests {
    use crate::book::OrderBook;
    use crate::engine::match_order;
    use crate::order::{Order, Side};

    #[test]
    fn full_fill() {
        let mut book = OrderBook::default();
        book.add_resting(Order::new(1, Side::Sell, 100, 10));
        let trades = match_order(&mut book, Order::new(2, Side::Buy, 100, 10));
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 10);
        assert!(book.best_ask().is_none());
    }

    #[test]
    fn partial_fill_rests() {
        let mut book = OrderBook::default();
        book.add_resting(Order::new(1, Side::Sell, 100, 4));
        let trades = match_order(&mut book, Order::new(2, Side::Buy, 100, 10));
        assert_eq!(trades[0].quantity, 4);
        // 6 unfilled now rests as a bid.
        assert_eq!(book.best_bid(), Some(100));
    }
}
