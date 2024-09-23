use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

/// A sliding window rate limiter
///
/// This rate limiter allows up to `limit` requests within a time window
/// defined by `window_duration`. The sliding window counter automatically
/// evicts old requests that exceed the window duration, ensuring that the
/// request count reflects only those within the current window.
///
/// The `SlidingWindowCounter` is safe for use in multithreaded applications, as
/// it leverages an `Arc` and `Mutex` to protect the internal queue of requests.
///
/// ## Example
///
/// ```rust
/// use tokio::time::Duration;
/// use limitr::window::SlidingWindowCounter;
/// # tokio_test::block_on(async {
///  // Create a rate limiter that allows 5 requests per 10-second window
///  let mut limiter = SlidingWindowCounter::new(5, Duration::from_secs(10));
///
///  // Attempt to consume a request
///  if limiter.try_consume().await {
///     println!("Request allowed.");
///  } else {
///     println!("Request rate-limited.");
///  }
/// # assert!(true);
/// # })
/// ```
///
/// # Fields
/// - `limit`: The maximum number of requests allowed in the time window.
/// - `window_duration`: The duration of the sliding window.
/// - `requests`: A deque of `Instant` timestamps, protected by a mutex,
///   representing when requests were made.
pub struct SlidingWindowCounter {
    limit: u32,
    window_duration: Duration,
    requests: Arc<Mutex<VecDeque<Instant>>>,
}

impl SlidingWindowCounter {
    /// Creates a new `SlidingWindowCounter` with the specified request limit
    /// and window duration.
    ///
    /// - `limit`: The maximum number of requests allowed within the window.
    /// - `window_duration`: The time window over which requests are counted.
    ///
    /// # Returns
    /// A new instance of `SlidingWindowCounter`.
    pub fn new(limit: u32, window_duration: Duration) -> Self {
        SlidingWindowCounter {
            limit,
            window_duration,
            requests: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Attempts to consume a request from the rate limiter.
    ///
    /// If the current number of requests within the time window is less than the
    /// limit, this method will allow the request and return `true`.
    /// If the limit has been reached, it will return `false`.
    ///
    /// This function clears out any requests that have expired based on the
    /// window duration.
    ///
    /// # Returns
    /// - `true` if the request is allowed.
    /// - `false` if the request is rate-limited.
    pub async fn try_consume(&mut self) -> bool {
        let now = Instant::now();
        let request = self.requests.clone();
        let mut requests = request.lock().await;

        // Remove old requests outside the window duration
        self.clear_old_requests(&mut requests, now).await;

        if requests.len() < self.limit as usize {
            // allow the request if under the limit
            requests.push_back(now);
            true
        } else {
            // reject request if limit is reached
            false
        }
    }

    /// Clears out requests that are older than the window duration.
    ///
    /// This function removes requests from the front of the deque that
    /// occurred before the current window. It is called every time
    /// `try_consume` is invoked to ensure that only requests within the
    /// valid window are counted.
    ///
    /// - `requests`: A mutable reference to the request deque.
    /// - `now`: The current time used for comparison with request timestamps.
    async fn clear_old_requests(&mut self, requests: &mut VecDeque<Instant>, now: Instant) {
        while let Some(request_time) = requests.front() {
            if now.duration_since(*request_time) > self.window_duration {
                requests.pop_front();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::window::SlidingWindowCounter;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_allows_requests_under_limit() {
        let mut limiter = SlidingWindowCounter::new(5, Duration::from_secs(10));

        for _ in 0..5 {
            assert!(limiter.try_consume().await, "Request should be allowed");
        }
    }

    #[tokio::test]
    async fn test_rate_limits_when_limit_exceeded() {
        let mut limiter = SlidingWindowCounter::new(3, Duration::from_secs(10));

        for _ in 0..3 {
            assert!(limiter.try_consume().await, "Request should be allowed");
        }

        // 4th request should be denied since limit is 3
        assert!(
            !limiter.try_consume().await,
            "Request should be rate-limited"
        );
    }

    #[tokio::test]
    async fn test_eviction_of_old_requests() {
        let mut limiter = SlidingWindowCounter::new(3, Duration::from_secs(2));

        for _ in 0..3 {
            assert!(limiter.try_consume().await, "Request should be allowed");
        }

        time::sleep(Duration::from_secs(3)).await;

        assert!(
            limiter.try_consume().await,
            "Request should be allowed after window expiration"
        );
    }

    #[tokio::test]
    async fn test_mixed_behavior() {
        let mut limiter = SlidingWindowCounter::new(5, Duration::from_secs(5));

        for _ in 0..4 {
            assert!(limiter.try_consume().await, "Request should be allowed");
        }

        assert!(limiter.try_consume().await, "Request should be allowed");

        assert!(
            !limiter.try_consume().await,
            "Request should be rate-limited"
        );

        time::sleep(Duration::from_secs(6)).await;

        assert!(
            limiter.try_consume().await,
            "Request should be allowed after window expiration"
        );
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let limiter = Arc::new(Mutex::new(SlidingWindowCounter::new(
            5,
            Duration::from_secs(10),
        )));

        let mut handles = vec![];

        for _ in 0..10 {
            let limiter_clone = limiter.clone();
            let handle = tokio::spawn(async move {
                let mut limiter = limiter_clone.lock().await;
                limiter.try_consume().await
            });
            handles.push(handle);
        }

        let mut allowed_count = 0;
        for handle in handles {
            if handle.await.unwrap() {
                allowed_count += 1;
            }
        }

        assert_eq!(
            allowed_count, 5,
            "Only 5 requests should be allowed due to the limit"
        );
    }
}
