use crate::book::OrderBook;
use crate::types::{Order, Side, Trade};

#[derive(Debug, Default)]
pub struct MatchingEngine {
    book: OrderBook,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            book: OrderBook::new(),
        }
    }

    pub fn process(&mut self, mut incoming: Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        let opposite_side = match incoming.side {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        };

        while incoming.remaining > 0 {
            let best_price = match opposite_side {
                Side::Buy => self.book.best_bid(),
                Side::Sell => self.book.best_ask(),
            };

            let price_crosses = match best_price {
                Some(price) => match incoming.side {
                    Side::Buy => incoming.price >= price,
                    Side::Sell => incoming.price <= price,
                },
                None => false,
            };

            if !price_crosses {
                break;
            }

            let resting_price = best_price.unwrap();
            let front = self.book.front_at(opposite_side, resting_price);
            if front.is_none() {
                break;
            }

            let resting = front.unwrap();
            let fill_qty = incoming.remaining.min(resting.remaining);
            incoming.remaining -= fill_qty;
            resting.remaining -= fill_qty;

            trades.push(Trade {
                buy_order_id: if incoming.side == Side::Buy {
                    incoming.id
                } else {
                    resting.id
                },
                sell_order_id: if incoming.side == Side::Sell {
                    incoming.id
                } else {
                    resting.id
                },
                price: resting.price,
                quantity: fill_qty,
            });

            if resting.remaining == 0 {
                self.book.pop_front_at(opposite_side, resting_price);
            }
        }

        if incoming.remaining > 0 {
            self.book.insert_resting(incoming);
        }

        trades
    }

    pub fn cancel(&mut self, order_id: u64) -> Option<Order> {
        self.book.remove(order_id)
    }

    pub fn book(&self) -> &OrderBook {
        &self.book
    }
}
