# hft-engine

A small Rust matching engine prototype for limit order book matching. This project implements a single-threaded order book and matching engine with support for resting orders, price-time priority, partial fills, and cancellations.

## What is included

- `src/types.rs`: shared order and trade types
- `src/book.rs`: limit order book storage and order management
- `src/engine.rs`: matching logic that produces trades and rests unfilled orders
- `tests/matching_tests.rs`: correctness tests covering matching behavior and edge cases

## Getting started

Build and run the tests:

```bash
cd hft-engine
cargo test
```

Run the synthetic latency demo:

```bash
cargo run --example latency_demo
```

Run the ring-buffer demo:

```bash
cargo run --example ring_buffer_demo
```

Run benchmark suites:

```bash
cargo bench
```

Run the C++ port demo and benchmark:

```bash
cd cpp
make
./bin/hft_engine_cpp
./bin/hft_engine_cpp_bench
./bin/hft_engine_cpp_spsc
```

## Results (local runs)

These numbers were gathered on the local machine used to develop this project. They are intended for illustrative comparison — absolute values will vary across CPUs, OSes, and compiler toolchains.

Rust single-threaded (latency_demo, 20k orders):
- throughput: 260,921 orders/sec
- p50 latency: 1.623 µs
- p99 latency: 10.715 µs

Rust SPSC demo (ring_buffer_demo, 10k orders):
- elapsed: 526.82 ms (end-to-end in the example harness)

C++ single-threaded (cpp/bin/hft_engine_cpp_bench, 20k orders):
- throughput: 651,765 orders/sec
- p50 latency: 630 ns
- p99 latency: 315 ns

C++ SPSC demo (cpp/bin/hft_engine_cpp_spsc, 10k orders):
- elapsed: 50.86 ms

Notes:
- The C++ binaries were built with g++ -O2; the Rust build used debug-mode examples/bench harness. For strict apples-to-apples, compile both in full-optimised/release modes and run under controlled conditions.
- The Rust SPSC demo in examples is a simple educational harness and not tuned for microsecond latency measurement; an exercise would be to instrument and benchmark the pipeline with proper warmup and isolation.

## Future phases

Planned future work (optional)

- Re-run all benchmarks in release mode and under controlled load, collecting detailed latency distributions (warmup, CPU affinity)
- Add Google Benchmark to the C++ suite and Criterion release-mode harness in Rust for consistent comparisons
- Typestate-based API for safer order transitions in Rust (we've sketched / partially implemented this as a follow-up)
- UDP-based ingestion and more realistic network measurement
- Polished README writeup explaining design tradeoffs and results
