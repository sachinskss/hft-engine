use hft_engine::{MatchingEngine, Order, Side};

fn main() {
    let mut engine = MatchingEngine::new();

    engine.process(Order::new(1, Side::Sell, 100, 10, 1));
    let trades = engine.process(Order::new(2, Side::Buy, 110, 15, 2));

    println!("Generated trades: {trades:#?}");
}
