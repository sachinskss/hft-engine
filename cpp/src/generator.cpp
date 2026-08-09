#include "generator.h"

OrderGenerator::OrderGenerator(Price mid_price, Price spread, uint64_t seed)
    : rng_(seed),
      mid_price_(mid_price),
      spread_(spread),
      next_id_(1),
      timestamp_(0),
      side_toggle_(false),
      spread_dist_(0, spread * 2),
      qty_dist_(1, 10) {}

Order OrderGenerator::next() {
    Price min_price = mid_price_ > spread_ ? mid_price_ - spread_ : 0;
    Price price = min_price + static_cast<Price>(spread_dist_(rng_));
    Order order{
        next_id_++,
        side_toggle_ ? Side::Buy : Side::Sell,
        price,
        static_cast<Quantity>(qty_dist_(rng_)),
        timestamp_++,
    };
    side_toggle_ = !side_toggle_;
    return order;
}
