#pragma once

#include <cstdint>
#include <random>

#include "types.h"

class OrderGenerator {
public:
    OrderGenerator(Price mid_price, Price spread, uint64_t seed);
    Order next();

private:
    std::mt19937_64 rng_;
    Price mid_price_;
    Price spread_;
    OrderId next_id_;
    uint64_t timestamp_;
    bool side_toggle_;
    std::uniform_int_distribution<uint64_t> spread_dist_;
    std::uniform_int_distribution<uint32_t> qty_dist_;
};
