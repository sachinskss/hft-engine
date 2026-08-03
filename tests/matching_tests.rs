use hft_engine::{MatchingEngine, Order, OrderBook, Side, Trade};

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
