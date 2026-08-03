use std::collections::{BTreeMap, HashMap, VecDeque};

use crate::types::{Order, OrderId, Price, Side};

#[derive(Debug, Default)]
pub struct OrderBook {
    bids: BTreeMap<Price, VecDeque<Order>>,
    asks: BTreeMap<Price, VecDeque<Order>>,
    index: HashMap<OrderId, (Side, Price)>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            index: HashMap::new(),
        }
    }

    pub fn best_bid(&self) -> Option<Price> {
        self.bids.keys().rev().next().copied()
    }

    pub fn best_ask(&self) -> Option<Price> {
        self.asks.keys().next().copied()
    }

    pub fn insert_resting(&mut self, order: Order) {
        let side_map = match order.side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        side_map
            .entry(order.price)
            .or_insert_with(VecDeque::new)
            .push_back(order.clone());
        self.index.insert(order.id, (order.side, order.price));
    }

    pub fn remove(&mut self, id: OrderId) -> Option<Order> {
        let &(side, price) = self.index.get(&id)?;
        let side_map = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let queue = side_map.get_mut(&price)?;
        let mut removed = None;
        let mut rebuilt_queue = VecDeque::with_capacity(queue.len());

        while let Some(order) = queue.pop_front() {
            if order.id == id {
                removed = Some(order);
                continue;
            }
            rebuilt_queue.push_back(order);
        }

        if removed.is_none() {
            *queue = rebuilt_queue;
            return None;
        }

        *queue = rebuilt_queue;
        if queue.is_empty() {
            side_map.remove(&price);
        }

        self.index.remove(&id);
        removed
    }

    pub fn front_at(&mut self, side: Side, price: Price) -> Option<&mut Order> {
        let side_map = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        side_map.get_mut(&price)?.front_mut()
    }

    pub fn pop_front_at(&mut self, side: Side, price: Price) -> Option<Order> {
        let side_map = match side {
            Side::Buy => &mut self.bids,
            Side::Sell => &mut self.asks,
        };

        let queue = side_map.get_mut(&price)?;
        let order = queue.pop_front();
        if queue.is_empty() {
            side_map.remove(&price);
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
