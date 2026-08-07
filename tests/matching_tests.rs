use hft_engine::{ConcurrentEngine, MatchingEngine, Order, OrderBook, Side, Trade};

fn order(id: u64, side: Side, price: u64, quantity: u32, timestamp: u64) -> Order {
    Order::new(id, side, price, quantity, timestamp)
}

#[test]
fn book_best_prices_empty() {
    let book = OrderBook::new();
    assert!(book.best_bid().is_none());
    assert!(book.best_ask().is_none());
}

#[test]
fn order_insertion_and_best_price() {
    let mut book = OrderBook::new();
    book.insert_resting(order(1, Side::Buy, 100, 10, 1));
    book.insert_resting(order(2, Side::Buy, 110, 5, 2));
    book.insert_resting(order(3, Side::Sell, 120, 7, 3));

    assert_eq!(book.best_bid(), Some(110));
    assert_eq!(book.best_ask(), Some(120));
}

#[test]
fn engine_matches_single_order_and_rests_remainder() {
    let mut engine = MatchingEngine::new();
    engine.process(order(1, Side::Sell, 100, 10, 1));
    let trades = engine.process(order(2, Side::Buy, 110, 15, 2));

    assert_eq!(trades.len(), 1);
    assert_eq!(
        trades[0],
        Trade {
            buy_order_id: 2,
            sell_order_id: 1,
            price: 100,
            quantity: 10
        }
    );
    assert_eq!(engine.book().best_bid(), Some(110));
}

#[test]
fn engine_preserves_time_priority_at_equal_prices() {
    let mut engine = MatchingEngine::new();
    engine.process(order(1, Side::Sell, 100, 5, 1));
    engine.process(order(2, Side::Sell, 100, 6, 2));

    let trades = engine.process(order(3, Side::Buy, 100, 8, 3));

    assert_eq!(trades.len(), 2);
    assert_eq!(
        trades[0],
        Trade {
            buy_order_id: 3,
            sell_order_id: 1,
            price: 100,
            quantity: 5
        }
    );
    assert_eq!(
        trades[1],
        Trade {
            buy_order_id: 3,
            sell_order_id: 2,
            price: 100,
            quantity: 3
        }
    );
    assert_eq!(engine.book().best_ask(), Some(100));
}

#[test]
fn engine_sweeps_multiple_price_levels() {
    let mut engine = MatchingEngine::new();
    engine.process(order(1, Side::Sell, 100, 4, 1));
    engine.process(order(2, Side::Sell, 105, 3, 2));
    engine.process(order(3, Side::Sell, 110, 2, 3));

    let trades = engine.process(order(4, Side::Buy, 110, 7, 4));

    assert_eq!(trades.len(), 2);
    assert_eq!(
        trades[0],
        Trade {
            buy_order_id: 4,
            sell_order_id: 1,
            price: 100,
            quantity: 4
        }
    );
    assert_eq!(
        trades[1],
        Trade {
            buy_order_id: 4,
            sell_order_id: 2,
            price: 105,
            quantity: 3
        }
    );
    assert_eq!(engine.book().best_ask(), Some(110));
}

#[test]
fn cancel_removes_resting_order() {
    let mut engine = MatchingEngine::new();
    engine.process(order(1, Side::Buy, 100, 10, 1));
    assert!(engine.cancel(1).is_some());
    assert!(engine.book().best_bid().is_none());
}

#[test]
fn order_generator_produces_alternating_sides_and_in_range_prices() {
    let mut generator = hft_engine::OrderGenerator::new(100, 10, 123);

    let first = generator.next().unwrap();
    let second = generator.next().unwrap();

    assert_eq!(first.id, 1);
    assert_eq!(second.id, 2);
    assert_ne!(first.side, second.side);
    assert!(first.price >= 90 && first.price <= 110);
    assert!(second.price >= 90 && second.price <= 110);
}

#[test]
fn latency_recorder_computes_percentiles_and_throughput() {
    use std::time::Duration;

    let mut recorder = hft_engine::LatencyRecorder::new();
    recorder.record(Duration::from_millis(10));
    recorder.record(Duration::from_millis(20));
    recorder.record(Duration::from_millis(30));

    assert_eq!(recorder.p50(), Duration::from_millis(20));
    assert_eq!(recorder.p99(), Duration::from_millis(30));
    assert!(recorder.throughput_per_sec(Duration::from_secs(1)) > 0.0);
}

#[test]
fn spsc_ring_buffer_pushes_and_pops_in_order() {
    use hft_engine::concurrent::SpscRingBuffer;

    let buffer = SpscRingBuffer::new(8);
    assert!(buffer.is_empty());

    buffer.push(1).unwrap();
    buffer.push(2).unwrap();
    assert!(!buffer.is_empty());
    assert_eq!(buffer.pop(), Some(1));
    assert_eq!(buffer.pop(), Some(2));
    assert!(buffer.is_empty());
}

#[test]
fn concurrent_engine_processes_orders() {
    let concurrent = ConcurrentEngine::new();
    let sender = concurrent.order_sender();
    let receiver = concurrent.trade_receiver();

    sender
        .send(order(1, Side::Sell, 100, 5, 1))
        .expect("send order");
    sender
        .send(order(2, Side::Buy, 110, 5, 2))
        .expect("send order");

    let trade = receiver.recv().expect("receive trade");

    assert_eq!(trade.buy_order_id, 2);
    assert_eq!(trade.sell_order_id, 1);
    assert_eq!(trade.price, 100);
    assert_eq!(trade.quantity, 5);
}
