use std::fmt;

pub type OrderId = u64;
pub type Price = u64;
pub type Quantity = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub price: Price,
    pub remaining: Quantity,
    pub timestamp: u64,
}

impl Order {
    pub fn new(id: OrderId, side: Side, price: Price, remaining: Quantity, timestamp: u64) -> Self {
        Self {
            id,
            side,
            price,
            remaining,
            timestamp,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Trade {
    pub buy_order_id: OrderId,
    pub sell_order_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
}
