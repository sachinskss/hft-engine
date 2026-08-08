#include "engine.h"

std::vector<Trade> MatchingEngine::process(Order incoming) {
    std::vector<Trade> trades;
    Side opposite = (incoming.side == Side::Buy) ? Side::Sell : Side::Buy;

    while (incoming.remaining > 0) {
        auto best_price_opt = (opposite == Side::Buy) ? book.best_bid() : book.best_ask();
        if (!best_price_opt) break;
        Price best_price = *best_price_opt;
        bool price_crosses = false;
        if (incoming.side == Side::Buy) price_crosses = incoming.price >= best_price;
        else price_crosses = incoming.price <= best_price;
        if (!price_crosses) break;

        auto front_opt = book.front_at(opposite, best_price);
        if (!front_opt) break;
        Order* resting = *front_opt;
        Quantity fill_qty = std::min(incoming.remaining, resting->remaining);
        incoming.remaining -= fill_qty;
        resting->remaining -= fill_qty;

        Trade t;
        t.buy_order_id = (incoming.side == Side::Buy) ? incoming.id : resting->id;
        t.sell_order_id = (incoming.side == Side::Sell) ? incoming.id : resting->id;
        t.price = resting->price;
        t.quantity = fill_qty;
        trades.push_back(t);

        if (resting->remaining == 0) {
            book.pop_front_at(opposite, best_price);
        }
    }

    if (incoming.remaining > 0) {
        book.insert_resting(incoming);
    }

    return trades;
}

std::optional<Order> MatchingEngine::cancel(OrderId id) {
    return book.remove(id);
}
