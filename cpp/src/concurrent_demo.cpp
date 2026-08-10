#include <atomic>
#include <chrono>
#include <iostream>
#include <thread>

#include "engine.h"
#include "ring_buffer.h"
#include "types.h"

int main() {
    const size_t order_count = 10000;
    SpscRingBuffer buffer(1024);
    std::atomic<bool> producer_done(false);

    auto producer = std::thread([&] {
        for (size_t i = 0; i < order_count; ++i) {
            Order order{
                static_cast<OrderId>(i + 1),
                (i % 2 == 0) ? Side::Buy : Side::Sell,
                static_cast<Price>(100 + (i % 5)),
                static_cast<Quantity>(1 + (i % 4)),
                static_cast<uint64_t>(i),
            };
            while (!buffer.push(order)) {
                std::this_thread::yield();
            }
        }
        producer_done.store(true, std::memory_order_release);
    });

    auto consumer = std::thread([&] {
        MatchingEngine engine;
        size_t processed = 0;
        size_t trades = 0;
        Order order;
        while (!producer_done.load(std::memory_order_acquire) || !buffer.is_empty()) {
            if (buffer.pop(order)) {
                auto result = engine.process(order);
                trades += result.size();
                ++processed;
            } else {
                std::this_thread::yield();
            }
        }
        std::cout << "consumer processed " << processed << " orders, produced " << trades << " trades\n";
    });

    auto start = std::chrono::steady_clock::now();
    producer.join();
    consumer.join();
    auto elapsed = std::chrono::steady_clock::now() - start;
    std::cout << "SPSC concurrent demo elapsed: " << std::chrono::duration<double, std::milli>(elapsed).count() << " ms\n";
    return 0;
}
