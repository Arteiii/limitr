use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use tokio::time::Duration;

/// A Fixed Window Counter rate limiter.
///
/// This implementation uses fixed time windows to limit the number of requests within each window.
/// It's simple to understand and implement, but can allow twice the rate of requests around window boundaries.
///
/// # Features
///
/// - Uses fixed time windows for rate limiting.
/// - Allows a specified number of requests within each time window.
/// - Automatically clears old windows to prevent memory growth.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
/// use limitr::window::FixedWindowCounter;
///
/// let mut counter = FixedWindowCounter::new(100, Duration::from_secs(60));
/// # tokio_test::block_on(async {
/// for _ in 0..100 {
///     assert!(counter.try_consume().await);
/// }
///
/// assert!(!counter.try_consume().await);
/// # })
/// ```
pub struct FixedWindowCounter {
    limit: u32,
    window_duration: Duration,
    windows: Mutex<HashMap<u64, u32>>,
}

impl FixedWindowCounter {
    /// Creates a new `FixedWindowCounter` with the specified `limit` and `window_duration`.
    ///
    /// * `limit`: The maximum number of requests allowed in each time window.
    /// * `window_duration`: The duration of each time window.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use limitr::window::FixedWindowCounter;
    ///
    /// let counter = FixedWindowCounter::new(100, Duration::from_secs(60)); // 100 requests per minute
    /// ```
    pub fn new(limit: u32, window_duration: Duration) -> Self {
        FixedWindowCounter {
            limit,
            window_duration,
            windows: Mutex::new(HashMap::new()),
        }
    }

    /// Attempts to consume a token from the current time window.
    ///
    /// Returns `true` if the request is allowed, and `false` if the limit has been reached for the current window.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use limitr::window::FixedWindowCounter;
    ///
    /// # tokio_test::block_on(async {
    /// let mut counter = FixedWindowCounter::new(5, Duration::from_secs(60));
    /// for _ in 0..5 {
    ///     assert!(counter.try_consume().await);
    /// }
    ///
    /// assert!(!counter.try_consume().await);
    /// # })
    /// ```
    pub async fn try_consume(&self) -> bool {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let current_window = current_time.as_secs() / self.window_duration.as_secs();
        let mut windows = self.windows.lock().await;

        let count = windows.entry(current_window).or_insert(0);
        if *count < self.limit {
            *count += 1;
            true
        } else {
            false
        }
    }

    /// Clears old time windows to prevent unbounded growth of the internal HashMap.
    ///
    /// This method should be called periodically to remove data for expired time windows.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::time::Duration;
    /// use limitr::window::FixedWindowCounter;
    ///
    /// # tokio_test::block_on(async {
    /// let mut counter = FixedWindowCounter::new(100, Duration::from_secs(60));
    /// // ... some time passes ...
    /// counter.clear_old_windows().await;
    /// # })
    /// ```
    pub async fn clear_old_windows(&self) {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let oldest_valid_window = current_time.as_secs() / self.window_duration.as_secs();
        let mut windows = self.windows.lock().await;

        windows.retain(|&window, _| window >= oldest_valid_window);
    }
}

#[cfg(test)]
mod tests {
    use crate::window::FixedWindowCounter;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn test_allows_requests_under_limit() {
        let counter = FixedWindowCounter::new(5, Duration::from_secs(60));

        for _ in 0..5 {
            assert!(counter.try_consume().await, "Request should be allowed");
        }

        assert!(
            !counter.try_consume().await,
            "Request should be rate-limited"
        );

        let windows = counter.windows.lock().await;
        let window_key = windows.keys().next().unwrap();
        assert_eq!(
            windows[window_key], 5,
            "Should have recorded 5 requests in the current window"
        );
    }

    #[tokio::test]
    async fn test_allows_new_window_requests() {
        let counter = FixedWindowCounter::new(3, Duration::from_secs(2));

        for _ in 0..3 {
            assert!(counter.try_consume().await, "Request should be allowed");
        }

        time::sleep(Duration::from_secs(3)).await;
        counter.clear_old_windows().await;

        assert!(
            counter.try_consume().await,
            "Request should be allowed in the new window"
        );

        let windows = counter.windows.lock().await;
        assert_eq!(windows.len(), 1, "There should be exactly 1 active window");
    }

    #[tokio::test]
    async fn test_clears_old_windows() {
        let counter = FixedWindowCounter::new(3, Duration::from_secs(2));

        for _ in 0..3 {
            assert!(counter.try_consume().await, "Request should be allowed");
        }

        counter.clear_old_windows().await;

        let windows = counter.windows.lock().await;
        assert!(
            !windows.is_empty(),
            "Windows should contain data after clearing"
        );

        drop(windows);

        time::sleep(Duration::from_secs(3)).await;

        counter.clear_old_windows().await;

        let windows = counter.windows.lock().await;
        assert!(
            windows.is_empty(),
            "Windows should be empty after clearing old windows"
        );
    }
}
