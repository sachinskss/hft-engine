use std::thread;
use std::time::Instant;

use hft_engine::concurrent::ConcurrentEngine;
use hft_engine::types::{Order, Side};

fn main() {
    let num_producers = 4usize;
    let orders_per_producer = 5000usize;

    let engine = ConcurrentEngine::new();
    let sender = engine.order_sender();
    let receiver = engine.trade_receiver();

    // start a thread to collect trades
    let collector = thread::spawn(move || {
        let mut count = 0usize;
        for _trade in receiver.iter() {
            count += 1;
            if count % 1000 == 0 {
                eprintln!("collected trades: {}", count);
            }
        }
        eprintln!("collector finished, total trades={}", count);
    });

    let start = Instant::now();
    let mut prod_handles = Vec::new();
    for p in 0..num_producers {
        let tx = sender.clone();
        let handle = thread::spawn(move || {
            let base_id = (p as u64) << 32;
            for i in 0..orders_per_producer {
                let id = base_id + i as u64;
                let side = if i % 2 == 0 { Side::Buy } else { Side::Sell };
                let price = 100 + (i % 5) as u64;
                let qty = 1 + (i % 4) as u32;
                let ts = i as u64;
                let order = Order::new(id, side, price, qty, ts);
                if tx.send(order).is_err() { break; }
            }
        });
        prod_handles.push(handle);
    }

    drop(sender); // close main sender
    for h in prod_handles { let _ = h.join(); }
    let elapsed = Instant::now() - start;
    eprintln!("producers finished in {} ms", elapsed.as_millis());

    // When all senders are dropped, engine worker will finish and close trades channel.
    let _ = collector.join();
}
