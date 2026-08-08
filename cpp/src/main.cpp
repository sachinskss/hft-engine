#include <iostream>
#include "engine.h"
#include "types.h"

int main() {
    MatchingEngine engine;
    // basic scenario: one resting sell, incoming buy
    Order sell{1, Side::Sell, 100, 10, 1};
    engine.book.insert_resting(sell);

    Order buy{2, Side::Buy, 110, 15, 2};
    auto trades = engine.process(buy);

    std::cout << "trades generated: " << trades.size() << "\n";
    for (auto &t : trades) {
        std::cout << "trade: buy=" << t.buy_order_id << " sell=" << t.sell_order_id << " price=" << t.price << " qty=" << t.quantity << "\n";
    }

    auto best_bid = engine.book.best_bid();
    if (best_bid) std::cout << "best_bid=" << *best_bid << "\n";
    else std::cout << "no best bid" << std::endl;

    return 0;
}
