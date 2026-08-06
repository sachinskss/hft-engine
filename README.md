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

Run benchmark suites:

```bash
cargo bench
```

## Future phases

Planned future work includes a C++ port and further concurrent ingestion paths.
