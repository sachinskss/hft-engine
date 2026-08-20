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

Run the release comparison harness for the Rust hot path versus the C++ implementation:

```bash
# Bash / zsh
./scripts/compare_release.sh

# PowerShell
./scripts/compare_release.ps1
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
- The C++ binaries were built with g++ -O2; the Rust bench harness used Criterion in the optimized (bench) profile. For strict apples-to-apples, compile both in fully-optimized/release modes and run under controlled conditions (CPU affinity, isolated cores, warmup runs).
- The Rust SPSC demo in examples is an educational harness and not tuned for microsecond latency measurement; for low-level latency you should instrument with a tracer and run dedicated microbenchmarks with warmup and process isolation.

Google Benchmark (C++)

A Google Benchmark target has been added (cpp/bench/bench_gbench.cpp) and a Makefile target `make bin/hft_engine_cpp_gbench`. This target attempts to use `pkg-config` to find an installed `benchmark` package. If `pkg-config` isn't available or Google Benchmark isn't installed, follow these steps locally to build it:

1. Install prerequisites (macOS example):
   - brew install cmake
   - brew install googletest
   - git clone https://github.com/google/benchmark.git cpp/third_party/benchmark
   - cd cpp/third_party/benchmark
   - git clone https://github.com/google/googletest.git
   - cmake -S . -B build -DCMAKE_BUILD_TYPE=Release -DBENCHMARK_ENABLE_GTEST_TESTS=OFF
   - cmake --build build -j
   - (then either install the library or use pkg-config to link)

2. Or install Google Benchmark system-wide (Linux distro/package manager) so `pkg-config --cflags --libs benchmark` works, then run in `cpp/`: `make bin/hft_engine_cpp_gbench`.

Criterion (Rust)

The Rust Criterion benches are present in `benches/engine_bench.rs`. Run the Criterion harness in the optimized bench profile with:

```bash
cargo bench
```

This repository was used to run `cargo bench` locally and produced optimized bench artifacts (bench profile) on the developer machine.

Portfolio writeup

Design rationale
- Keep a clear separation of concerns: `types` defines the vocabulary, `book` holds resting state, `engine` contains matching logic, `generator` produces synthetic orders, and `stats` records latency/throughput. This layering makes each component independently testable and benchmarkable.
- Start with a correct single-threaded impl (easier to reason about and unit test) then add concurrent ingestion at the boundary — the realistic production pattern is a single-owner book and matching core with lock-free or low-lock ingress.
- Implemented two concurrent ingestion approaches:
  - crossbeam MPMC producer(s) → single consumer owning the `MatchingEngine` (practical and easy to reason about)
  - SPSC ring buffer (hand-rolled) to show low-level atomics-based ingestion when a single producer/consumer can be used
- C++ port implemented to allow language-level tradeoff comparison (allocation strategies, layout, and compiler optimisations)

Architecture (high-level ASCII diagram)

 generator (Rust/C++)  --->  [ingestion queue: crossbeam / ring buffer]  --->  MatchingEngine (single-owner)  --->  OrderBook
                                                       |
                                                       v
                                                     Trades -> stats / collector

Key code excerpts

- Rust matching loop (simplified):

```rust
while incoming.remaining > 0 {
    let best_price = opposite_side_best_price();
    if !price_crosses(incoming, best_price) { break; }
    let resting = book.front_at(opposite, best_price).unwrap();
    let fill = incoming.remaining.min(resting.remaining);
    incoming.remaining -= fill;
    resting.remaining -= fill;
    emit_trade(...);
    if resting.remaining == 0 { book.pop_front_at(opposite, best_price); }
}
if incoming.remaining > 0 { book.insert_resting(incoming); }
```

- C++ matching loop mirrors the same control flow and data structures (std::map per side, std::deque per price level) so benchmarks compare algorithmic differences, not business logic.

How to reproduce (recommended)

1. Rust (Criterion):
   - `cargo bench`

2. Rust examples:
   - `cargo run --example latency_demo`
   - `cargo run --example ring_buffer_demo`

3. C++:
   - `cd cpp && make`
   - `./bin/hft_engine_cpp` (demo)
   - `./bin/hft_engine_cpp_bench` (simple harness)
   - `./bin/hft_engine_cpp_spsc` (SPSC concurrent demo)
   - If Google Benchmark is available, `make bin/hft_engine_cpp_gbench` and run that binary for benchmark results

Further notes
- The repo includes unit tests for Rust and lightweight examples for both Rust and C++ to demonstrate behavior and gather performance data.
- If you want, I can run a repeatable, script-driven set of release-mode runs that:
  1) Build Rust/C++ in release
  2) Run each harness multiple times with warmup, collect p50/p95/p99 and throughput
  3) Produce a side-by-side markdown report embedded in the README with tables and recommendations.

