//! Leaky Bucket Algorithm Implementation
//!
//! The Leaky Bucket algorithm ensures that requests are processed at a steady rate. It
//! "leaks" requests at a constant rate, regardless of incoming request burstiness.
//!
//! ## How it Works
//! The bucket has a fixed capacity. Requests can be made up to the bucket's capacity, and then
//! the requests are allowed to leak out at a steady rate. If the bucket is empty, incoming requests are denied.
//!
//! ## Example
//!
//! ```rust
//! use limitr::bucket::LeakyBucket;
//! use tokio::time::{sleep, Duration};
//!
//! # tokio_test::block_on(async {
//!  let mut bucket = LeakyBucket::new(10, 2);
//!
//!     for i in 0..15 {
//!         if bucket.try_consume().await {
//!             println!("Request {} succeeded.", i + 1);
//!         } else {
//!             println!("Request {} failed, bucket is empty.", i);
//!         }
//!         sleep(Duration::from_millis(500)).await;
//!     }
//! # assert!(true);
//! # })
//! ```

use tokio::time::Instant;
use tracing::trace;

/// The `LeakyBucket` struct manages rate-limiting by allowing a steady rate of requests.
pub struct LeakyBucket {
    /// Total capacity of the bucket
    capacity: usize,
    /// How many requests are left
    remaining: usize,
    /// How many tokens to leak per second
    leak_rate: usize,
    /// Last time the bucket was checked
    last_checked: Instant,
}

impl LeakyBucket {
    /// Creates a new `LeakyBucket` with the given capacity and leak rate.
    ///
    /// ## Parameters
    /// - `capacity`: The maximum number of requests the bucket can hold.
    /// - `leak_rate`: The rate at which requests are leaked from the bucket, in requests per second.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use limitr::bucket::LeakyBucket;
    ///
    /// let bucket = LeakyBucket::new(10, 2);
    /// # assert!(true);
    /// ```
    pub fn new(capacity: usize, leak_rate: usize) -> Self {
        LeakyBucket {
            capacity,
            remaining: capacity,
            leak_rate,
            last_checked: Instant::now(),
        }
    }

    /// Tries to consume one token from the bucket.
    ///
    /// Returns `true` if successful, otherwise returns `false` if the bucket is empty.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use limitr::bucket::LeakyBucket;
    /// use tokio::time::Duration;
    /// # tokio_test::block_on(async {
    ///  let mut bucket = LeakyBucket::new(10, 2);
    ///
    ///  if bucket.try_consume().await {
    ///     println!("Request succeeded.");
    ///  } else {
    ///     println!("Request failed, bucket is empty.");
    ///  }
    /// # })
    /// ```
    pub async fn try_consume(&mut self) -> bool {
        self.leak().await;
        if self.remaining > 0 {
            self.remaining -= 1;
            trace!("Request processed, remaining tokens: {}", self.remaining);
            true
        } else {
            trace!("Request denied, bucket is empty.");
            false
        }
    }

    /// Leaks tokens based on the elapsed time since the last check.
    async fn leak(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_checked).as_secs() as usize;
        let leak_amount = elapsed * self.leak_rate;

        if leak_amount > 0 {
            self.remaining = (self.remaining + leak_amount).min(self.capacity);
            self.last_checked = now;
            trace!(
                "Leaked {} tokens, current capacity: {}",
                leak_amount,
                self.remaining
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bucket::LeakyBucket;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_new_bucket() {
        let mut bucket = LeakyBucket::new(10, 2);

        for _ in 0..10 {
            assert!(bucket.try_consume().await);
        }
        assert!(!bucket.try_consume().await);
    }

    #[tokio::test]
    async fn test_consume_success() {
        let mut bucket = LeakyBucket::new(5, 1);
        assert!(bucket.try_consume().await);

        for _ in 0..4 {
            assert!(bucket.try_consume().await);
        }
        assert!(!bucket.try_consume().await);
    }

    #[tokio::test]
    async fn test_consume_empty_bucket() {
        let mut bucket = LeakyBucket::new(1, 1);
        assert!(bucket.try_consume().await);
        assert!(!bucket.try_consume().await);
    }

    #[tokio::test]
    async fn test_leak() {
        let mut bucket = LeakyBucket::new(1, 1);
        assert!(bucket.try_consume().await);
        assert!(!bucket.try_consume().await);
        sleep(Duration::from_secs(2)).await;
        assert!(bucket.try_consume().await);
    }

    #[tokio::test]
    async fn test_leak_up_to_capacity() {
        let mut bucket = LeakyBucket::new(5, 2);

        // consume all
        for _ in 0..5 {
            assert!(bucket.try_consume().await);
        }
        assert!(!bucket.try_consume().await);
        sleep(Duration::from_secs(3)).await;

        // After 3 seconds, 6 tokens should have leaked (2 per second), but capacity is 5
        for _ in 0..5 {
            assert!(bucket.try_consume().await);
        }
        assert!(!bucket.try_consume().await);
    }

    #[tokio::test]
    async fn test_multiple_consume_and_leak() {
        let mut bucket = LeakyBucket::new(5, 1);
        for _ in 0..5 {
            assert!(bucket.try_consume().await);
        }
        assert!(!bucket.try_consume().await);
        sleep(Duration::from_secs(2)).await;
        assert!(bucket.try_consume().await);
        assert!(bucket.try_consume().await);
        assert!(!bucket.try_consume().await);
    }
}
