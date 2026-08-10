#pragma once

#include <atomic>
#include <cstddef>
#include <vector>

#include "types.h"

class SpscRingBuffer {
public:
    explicit SpscRingBuffer(size_t capacity)
        : capacity_(capacity + 1),
          buffer_(capacity_),
          head_(0),
          tail_(0) {}

    bool push(const Order& order) {
        size_t tail = tail_.load(std::memory_order_relaxed);
        size_t next = increment(tail);
        if (next == head_.load(std::memory_order_acquire)) {
            return false;
        }
        buffer_[tail] = order;
        tail_.store(next, std::memory_order_release);
        return true;
    }

    bool pop(Order& order) {
        size_t head = head_.load(std::memory_order_relaxed);
        if (head == tail_.load(std::memory_order_acquire)) {
            return false;
        }
        order = buffer_[head];
        head_.store(increment(head), std::memory_order_release);
        return true;
    }

    bool is_empty() const {
        return head_.load(std::memory_order_acquire) == tail_.load(std::memory_order_acquire);
    }

private:
    size_t increment(size_t index) const {
        return (index + 1) % capacity_;
    }

    const size_t capacity_;
    std::vector<Order> buffer_;
    std::atomic<size_t> head_;
    std::atomic<size_t> tail_;
};
