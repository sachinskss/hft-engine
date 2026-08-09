#include "stats.h"
#include <algorithm>
#include <cmath>

void LatencyRecorder::record(Duration duration) {
    samples_.push_back(duration);
}

LatencyRecorder::Duration LatencyRecorder::p50() const {
    if (samples_.empty()) return Duration::zero();
    auto sorted = samples_;
    std::sort(sorted.begin(), sorted.end());
    size_t idx = static_cast<size_t>(std::lround((sorted.size() - 1) * 0.50));
    return sorted[idx];
}

LatencyRecorder::Duration LatencyRecorder::p99() const {
    if (samples_.empty()) return Duration::zero();
    auto sorted = samples_;
    size_t idx = static_cast<size_t>(std::lround((sorted.size() - 1) * 0.99));
    return sorted[idx];
}

double LatencyRecorder::throughput_per_sec(Duration elapsed) const {
    if (elapsed.count() == 0) return 0.0;
    return static_cast<double>(samples_.size()) / std::chrono::duration<double>(elapsed).count();
}

size_t LatencyRecorder::sample_count() const {
    return samples_.size();
}
