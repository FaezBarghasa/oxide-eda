use std::time::{Duration, Instant};

/// Exponential backoff calculator with randomized jitter for distributor scraping and crawling.
#[derive(Debug, Clone)]
pub struct BackoffConfig {
    pub base_ms: u64,
    pub max_ms: u64,
    pub jitter_factor: f64,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            base_ms: 250,
            max_ms: 10_000,
            jitter_factor: 0.25,
        }
    }
}

impl BackoffConfig {
    /// Calculates backoff duration for a given retry attempt:
    /// t_backoff = min(t_max, t_base * 2^retry) ± jitter
    pub fn calculate_delay(&self, retry: u32) -> Duration {
        let exp = 2u64.saturating_pow(retry);
        let raw_ms = (self.base_ms.saturating_mul(exp)).min(self.max_ms);

        // Pseudo-random deterministic jitter based on timestamp
        let now_nanos = Instant::now().elapsed().as_nanos() as f64;
        let pseudo_rand = ((now_nanos.sin() + 1.0) / 2.0) * 2.0 - 1.0; // [-1.0, 1.0]
        let jitter_ms = (raw_ms as f64 * self.jitter_factor * pseudo_rand) as i64;

        let final_ms = (raw_ms as i64 + jitter_ms).max(10) as u64;
        Duration::from_millis(final_ms)
    }
}

/// Token bucket rate limiter for distributor APIs.
#[derive(Debug)]
pub struct TokenBucket {
    pub capacity: f64,
    pub refill_rate_per_sec: f64,
    pub tokens: f64,
    pub last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate_per_sec: f64) -> Self {
        Self {
            capacity,
            refill_rate_per_sec,
            tokens: capacity,
            last_refill: Instant::now(),
        }
    }

    pub fn try_acquire(&mut self, required_tokens: f64) -> bool {
        self.refill();
        if self.tokens >= required_tokens {
            self.tokens -= required_tokens;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed_secs = now.duration_since(self.last_refill).as_secs_f64();
        self.last_refill = now;
        self.tokens = (self.tokens + elapsed_secs * self.refill_rate_per_sec).min(self.capacity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_increases_monotonically_bounded() {
        let config = BackoffConfig {
            base_ms: 100,
            max_ms: 2000,
            jitter_factor: 0.1,
        };
        let d0 = config.calculate_delay(0);
        let d1 = config.calculate_delay(1);
        let d5 = config.calculate_delay(5);

        assert!(d0.as_millis() >= 50);
        assert!(d1.as_millis() >= d0.as_millis() / 2);
        assert!(d5.as_millis() <= 2500);
    }

    #[test]
    fn token_bucket_acquire_and_refill() {
        let mut bucket = TokenBucket::new(5.0, 10.0);
        assert!(bucket.try_acquire(3.0));
        assert!(bucket.try_acquire(2.0));
        assert!(!bucket.try_acquire(1.0));
    }
}
