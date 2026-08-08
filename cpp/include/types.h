#pragma once

#include <cstdint>

using OrderId = uint64_t;
using Price = uint64_t;
using Quantity = uint32_t;

enum class Side { Buy, Sell };

struct Order {
    OrderId id;
    Side side;
    Price price;
    Quantity remaining;
    uint64_t timestamp;
};

struct Trade {
    OrderId buy_order_id;
    OrderId sell_order_id;
    Price price;
    Quantity quantity;
};
