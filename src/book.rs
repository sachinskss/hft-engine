use std::collections::{HashMap, VecDeque};

use crate::types::{Order, OrderId, Price, Side};

#[derive(Debug, Default)]
struct PriceLevel {
    price: Price,
    orders: VecDeque<Order>,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    bids: Vec<PriceLevel>,
    asks: Vec<PriceLevel>,
    index: HashMap<OrderId, (Side, Price)>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn side_levels_mut(&mut self, side: Side) -> &mut Vec<PriceLevel> {
        match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        }
    }

    fn find_level_index(levels: &[PriceLevel], price: Price) -> Option<usize> {
        levels.iter().position(|level| level.price == price)
    }

    pub fn best_bid(&self) -> Option<Price> {
        self.bids.iter().map(|level| level.price).max()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.iter().map(|level| level.price).min()
    }

    pub fn insert_resting(&mut self, order: Order) {
        let (id, side, price) = (order.id, order.side, order.price);
        let levels = self.side_levels_mut(side);
        match Self::find_level_index(levels, price) {
            Some(index) => {
                levels[index].orders.push_back(order);
            }
            None => {
                levels.push(PriceLevel {
                    price,
                    orders: VecDeque::from([order]),
                });
            }
        }
        self.index.insert(id, (side, price));
    }

    pub fn remove(&mut self, id: OrderId) -> Option<Order> {
        let &(side, price) = self.index.get(&id)?;
        let levels = self.side_levels_mut(side);
        let level_index = Self::find_level_index(levels, price)?;

        let queue = &mut levels[level_index].orders;
        let mut removed = None;
        let mut rebuilt_queue = VecDeque::with_capacity(queue.len());

        while let Some(order) = queue.pop_front() {
            if order.id == id {
                removed = Some(order);
                continue;
            }
            rebuilt_queue.push_back(order);
        }

        *queue = rebuilt_queue;
        if queue.is_empty() {
            levels.remove(level_index);
        }

        self.index.remove(&id);
        removed
    }

    pub fn front_at(&mut self, side: Side, price: Price) -> Option<&mut Order> {
        let levels = self.side_levels_mut(side);
        let level_index = Self::find_level_index(levels, price)?;
        levels[level_index].orders.front_mut()
    }

    pub fn pop_front_at(&mut self, side: Side, price: Price) -> Option<Order> {
        let levels = self.side_levels_mut(side);
        let level_index = Self::find_level_index(levels, price)?;

        let order = levels[level_index].orders.pop_front();
        if levels[level_index].orders.is_empty() {
            levels.remove(level_index);
        }

        if let Some(order) = &order {
            self.index.remove(&order.id);
        }

        order
    }

    pub fn is_empty(&self) -> bool {
        self.bids.is_empty() && self.asks.is_empty()
    }
}
