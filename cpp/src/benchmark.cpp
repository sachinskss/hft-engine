#include <chrono>
#include <iostream>
#include <vector>

#include "engine.h"
#include "generator.h"
#include "stats.h"

int main() {
    const size_t order_count = 20000;
    OrderGenerator generator(100, 10, 12345);
    MatchingEngine engine;
    LatencyRecorder recorder;
    std::vector<Order> orders;
    orders.reserve(order_count);

    for (size_t i = 0; i < order_count; ++i) {
        orders.push_back(generator.next());
    }

    auto start = std::chrono::steady_clock::now();
    for (auto& order : orders) {
        auto begin = std::chrono::steady_clock::now();
        engine.process(order);
        recorder.record(std::chrono::duration_cast<LatencyRecorder::Duration>(std::chrono::steady_clock::now() - begin));
    }
    auto elapsed = std::chrono::steady_clock::now() - start;

    std::cout << "processed " << recorder.sample_count() << " orders\n";
    std::cout << "throughput = " << recorder.throughput_per_sec(std::chrono::duration_cast<LatencyRecorder::Duration>(elapsed)) << " orders/sec\n";
    std::cout << "p50 latency = " << recorder.p50().count() << " ns\n";
    std::cout << "p99 latency = " << recorder.p99().count() << " ns\n";
    return 0;
}
