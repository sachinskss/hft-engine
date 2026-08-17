#include <benchmark/benchmark.h>
#include "engine.h"
#include "generator.h"

static void BM_Engine_Serialize(benchmark::State& state) {
    MatchingEngine engine;
    OrderGenerator gen(100, 10, 42);
    for (auto _ : state) {
        auto o = gen.next();
        engine.process(o);
    }
}
BENCHMARK(BM_Engine_Serialize);
BENCHMARK_MAIN();
