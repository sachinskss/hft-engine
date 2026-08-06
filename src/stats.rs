use std::time::Duration;

#[derive(Debug, Default)]
pub struct LatencyRecorder {
    samples: Vec<Duration>,
}

impl LatencyRecorder {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn record(&mut self, d: Duration) {
        self.samples.push(d);
    }

    pub fn p50(&self) -> Duration {
        self.percentile(0.50)
    }

    pub fn p99(&self) -> Duration {
        self.percentile(0.99)
    }

    pub fn throughput_per_sec(&self, elapsed: Duration) -> f64 {
        if elapsed.is_zero() {
            return 0.0;
        }
        self.samples.len() as f64 / elapsed.as_secs_f64()
    }

    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    fn percentile(&self, fraction: f64) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }

        let mut sorted = self.samples.clone();
        sorted.sort_unstable();
        let sample_count = sorted.len();
        let rank = ((sample_count - 1) as f64 * fraction).round() as usize;
        sorted[rank]
    }
}
