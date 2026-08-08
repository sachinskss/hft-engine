#pragma once

#include <map>
#include <deque>
#include <unordered_map>
#include <optional>

#include "types.h"

struct OrderBook {
    std::map<Price, std::deque<Order>> bids; // ascending keys; use rbegin for best bid
    std::map<Price, std::deque<Order>> asks; // ascending keys; begin() for best ask
    std::unordered_map<OrderId, std::pair<Side, Price>> index;

    OrderBook() = default;

    std::optional<Price> best_bid() const;
    std::optional<Price> best_ask() const;
    void insert_resting(const Order& order);
    std::optional<Order> remove(OrderId id);
    std::optional<Order*> front_at(Side side, Price price);
    std::optional<Order> pop_front_at(Side side, Price price);
    bool is_empty() const;
};
