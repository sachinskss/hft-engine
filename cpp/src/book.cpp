#include "book.h"

#include <algorithm>

std::optional<Price> OrderBook::best_bid() const {
    if (bids.empty()) return std::nullopt;
    return bids.rbegin()->first;
}

std::optional<Price> OrderBook::best_ask() const {
    if (asks.empty()) return std::nullopt;
    return asks.begin()->first;
}

void OrderBook::insert_resting(const Order& order) {
    auto &side_map = (order.side == Side::Buy) ? bids : asks;
    side_map[order.price].push_back(order);
    index[order.id] = {order.side, order.price};
}

std::optional<Order> OrderBook::remove(OrderId id) {
    auto it = index.find(id);
    if (it == index.end()) return std::nullopt;
    Side side = it->second.first;
    Price price = it->second.second;
    auto &side_map = (side == Side::Buy) ? bids : asks;
    auto qm = side_map.find(price);
    if (qm == side_map.end()) return std::nullopt;

    std::deque<Order> &dq = qm->second;
    std::deque<Order> newdq;
    std::optional<Order> removed;
    while (!dq.empty()) {
        Order o = dq.front(); dq.pop_front();
        if (o.id == id) { removed = o; break; }
        newdq.push_back(o);
    }
    // restore remainder
    while (!dq.empty()) { newdq.push_back(dq.front()); dq.pop_front(); }
    dq = std::move(newdq);

    if (dq.empty()) side_map.erase(price);
    index.erase(it);
    return removed;
}

std::optional<Order*> OrderBook::front_at(Side side, Price price) {
    auto &side_map = (side == Side::Buy) ? bids : asks;
    auto it = side_map.find(price);
    if (it == side_map.end() || it->second.empty()) return std::nullopt;
    return &it->second.front();
}

std::optional<Order> OrderBook::pop_front_at(Side side, Price price) {
    auto &side_map = (side == Side::Buy) ? bids : asks;
    auto it = side_map.find(price);
    if (it == side_map.end() || it->second.empty()) return std::nullopt;
    Order o = it->second.front();
    it->second.pop_front();
    if (it->second.empty()) side_map.erase(it);
    index.erase(o.id);
    return o;
}

bool OrderBook::is_empty() const {
    return bids.empty() && asks.empty();
}
