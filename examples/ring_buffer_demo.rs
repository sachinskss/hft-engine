use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Instant;

use hft_engine::concurrent::SpscRingBuffer;
use hft_engine::types::{Order, Side};

fn main() {
    let buffer = Arc::new(SpscRingBuffer::new(1024));
    let finished = Arc::new(AtomicBool::new(false));
    let producer_buffer = Arc::clone(&buffer);
    let consumer_finished = Arc::clone(&finished);

    let producer_finished = Arc::clone(&finished);
    let producer = thread::spawn(move || {
        let mut next_id = 1u64;
        for i in 0..10_000 {
            let order = Order::new(
                next_id,
                if i % 2 == 0 { Side::Buy } else { Side::Sell },
                100 + (i % 5) as u64,
                1 + (i % 4) as u32,
                i as u64,
            );
            next_id += 1;

            while producer_buffer.push(order.clone()).is_err() {
                thread::yield_now();
            }
        }

        producer_finished.store(true, Ordering::Release);
    });

    let consumer = thread::spawn(move || {
        let mut processed = 0usize;
        let mut trades = 0usize;
        let mut engine = hft_engine::engine::MatchingEngine::new();

        while !consumer_finished.load(Ordering::Acquire) || !buffer.is_empty() {
            if let Some(order) = buffer.pop() {
                let result = engine.process(order);
                trades += result.len();
                processed += 1;
            } else {
                thread::yield_now();
            }
        }

        eprintln!(
            "SPSC consumer processed {} orders and produced {} trades",
            processed, trades
        );
    });

    let start = Instant::now();
    let _ = producer.join();
    let _ = consumer.join();
    let elapsed = start.elapsed();

    println!("SPSC demo elapsed: {:.2?}", elapsed);
}
