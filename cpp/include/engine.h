#pragma once

#include <vector>

#include "book.h"
#include "types.h"

struct MatchingEngine {
    OrderBook book;

    MatchingEngine() = default;

    std::vector<Trade> process(Order incoming);
    std::optional<Order> cancel(OrderId id);
};
