#pragma once

#include <chrono>
#include <vector>

class LatencyRecorder {
public:
    using Duration = std::chrono::nanoseconds;

    void record(Duration duration);
    Duration p50() const;
    Duration p99() const;
    double throughput_per_sec(Duration elapsed) const;
    size_t sample_count() const;

private:
    std::vector<Duration> samples_;
};
