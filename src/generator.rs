use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::types::{Order, OrderId, Price, Side};

/// A synthetic order generator for benchmarking and testing.
#[derive(Debug)]
pub struct OrderGenerator {
    rng: SmallRng,
    mid_price: Price,
    spread: Price,
    next_id: OrderId,
    timestamp: u64,
    side_toggle: bool,
}

impl OrderGenerator {
    pub fn new(mid_price: Price, spread: Price, seed: u64) -> Self {
        Self {
            rng: SmallRng::seed_from_u64(seed),
            mid_price,
            spread,
            next_id: 1,
            timestamp: 0,
            side_toggle: false,
        }
    }

    fn next_price(&mut self) -> Price {
        let lower = self.mid_price.saturating_sub(self.spread);
        let range = (self.spread * 2).saturating_add(1);
        lower + self.rng.gen_range(0..range)
    }
}

impl Iterator for OrderGenerator {
    type Item = Order;

    fn next(&mut self) -> Option<Self::Item> {
        let order = Order::new(
            self.next_id,
            if self.side_toggle {
                Side::Buy
            } else {
                Side::Sell
            },
            self.next_price(),
            self.rng.gen_range(1..=10),
            self.timestamp,
        );

        self.next_id = self.next_id.wrapping_add(1);
        self.timestamp = self.timestamp.wrapping_add(1);
        self.side_toggle = !self.side_toggle;

        Some(order)
    }
}
