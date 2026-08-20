#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "============================================================"
echo " Rust release benchmark"
echo "============================================================"
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release
cargo run --release --example latency_demo

echo

echo "============================================================"
echo " C++ release benchmark"
echo "============================================================"
cd cpp
mkdir -p bin

CPP_BIN=""
if command -v g++ >/dev/null 2>&1; then
  CPP_BIN="g++"
elif command -v clang++ >/dev/null 2>&1; then
  CPP_BIN="clang++"
else
  echo "No C++ compiler available (g++/clang++) in PATH. Install MinGW or LLVM/clang++ and retry."
  exit 1
fi

"$CPP_BIN" -std=c++17 -O2 -Wall -Wextra -Iinclude -o bin/hft_engine_cpp_bench src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp
./bin/hft_engine_cpp_bench
