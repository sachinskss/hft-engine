use std::time::Instant;

use hft_engine::{LatencyRecorder, MatchingEngine, OrderGenerator};

fn main() {
    let mut engine = MatchingEngine::new();
    let mut generator = OrderGenerator::new(100, 10, 42);
    let mut recorder = LatencyRecorder::new();
    let orders: Vec<_> = (0..20_000).map(|_| generator.next().unwrap()).collect();

    let start = Instant::now();
    for order in orders {
        let begin = Instant::now();
        engine.process(order);
        recorder.record(begin.elapsed());
    }
    let elapsed = start.elapsed();

    println!("processed {} orders", recorder.sample_count());
    println!(
        "throughput = {:.2} orders/sec",
        recorder.throughput_per_sec(elapsed)
    );
    println!("p50 latency = {:?}", recorder.p50());
    println!("p99 latency = {:?}", recorder.p99());
}
