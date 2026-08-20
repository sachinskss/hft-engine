$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

Write-Host "============================================================"
Write-Host " Rust release benchmark"
Write-Host "============================================================"
$env:PATH += ';C:\Users\PC\.cargo\bin'
cargo build --release
cargo run --release --example latency_demo

Write-Host ""
Write-Host "============================================================"
Write-Host " C++ release benchmark"
Write-Host "============================================================"
Set-Location cpp
New-Item -ItemType Directory -Force -Path bin | Out-Null

$compiler = $null
if (Get-Command g++ -ErrorAction SilentlyContinue) { $compiler = 'g++' }
elseif (Get-Command clang++ -ErrorAction SilentlyContinue) { $compiler = 'clang++' }
else { throw 'No C++ compiler available (g++/clang++). Install MinGW or LLVM/clang++ and retry.' }

& $compiler -std=c++17 -O2 -Wall -Wextra -Iinclude -o bin/hft_engine_cpp_bench src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp
& ./bin/hft_engine_cpp_bench
