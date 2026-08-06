use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

use hft_engine::{LatencyRecorder, MatchingEngine, OrderGenerator};

fn bench_single_threaded_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_threaded_throughput");
    for &order_count in &[1_000usize, 10_000, 50_000] {
        group.throughput(Throughput::Elements(order_count as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(order_count),
            &order_count,
            |b, &_size| {
                b.iter_custom(|iters| {
                    let mut generator = OrderGenerator::new(100, 10, 0);
                    let mut engine = MatchingEngine::new();
                    let mut total = Duration::ZERO;

                    for _ in 0..iters {
                        let order = generator.next().unwrap();
                        let start = Instant::now();
                        black_box(engine.process(order));
                        total += start.elapsed();
                    }

                    total
                })
            },
        );
    }
    group.finish();
}

fn bench_latency_distribution(c: &mut Criterion) {
    let mut generator = OrderGenerator::new(100, 10, 1);
    let mut engine = MatchingEngine::new();
    let mut recorder = LatencyRecorder::new();
    let orders: Vec<_> = (0..10_000).map(|_| generator.next().unwrap()).collect();

    c.bench_function("latency_distribution", |b| {
        b.iter(|| {
            for order in orders.iter().cloned() {
                let start = Instant::now();
                black_box(engine.process(order));
                recorder.record(start.elapsed());
            }
        })
    });

    let p50 = recorder.p50();
    let p99 = recorder.p99();
    println!(
        "latency p50={:?} p99={:?} for {} samples",
        p50,
        p99,
        recorder.sample_count()
    );
}

criterion_group!(
    benches,
    bench_single_threaded_throughput,
    bench_latency_distribution
);
criterion_main!(benches);
