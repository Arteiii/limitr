use std::time::Instant;
use tracing::trace;

/// An asynchronous Token Bucket rate limiter.
///
/// This implementation refills tokens based on the elapsed time since the last refill
/// and allows a burst of requests up to the capacity of the bucket. When the bucket runs out
/// of tokens, further requests are delayed until tokens are refilled.
///
/// # Features
///
/// - Supports asynchronous operations using `tokio`.
/// - Provides detailed tracing for debugging via the `tracing` crate.
///
/// # Example
///
/// ```rust
/// use tokio::time::sleep;
/// use std::time::Duration;
/// use limitr::bucket::TokenBucket;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a token bucket with a capacity of 10 tokens and refill rate of 5 tokens per second
///     let mut bucket = TokenBucket::new(10, 5);
///
///     // Simulate 20 requests with a delay of 500ms between each
///     for i in 0..20 {
///         if bucket.try_consume(2).await {
///             println!("Request {} succeeded, tokens left: {}", i + 1, bucket.available_tokens().await);
///         } else {
///             println!("Request {} failed, not enough tokens.", i + 1);
///         }
///         sleep(Duration::from_millis(500)).await;
///     }
/// }
/// ```
///
/// # Tracing
///
/// The implementation uses `tracing` for logging at `info` and `debug` levels. To capture logs, you need to set up a subscriber:
///
/// ```rust
/// use tracing_subscriber;
///
/// tracing_subscriber::fmt::init();
/// // Your code here...
/// ```
///
pub struct TokenBucket {
    /// Maximum number of tokens in the bucket
    capacity: u64,
    /// Current number of tokens
    tokens: u64,
    /// Tokens added per second
    refill_rate: u64,
    /// Time of last token refill
    last_refill: Instant,
}

impl TokenBucket {
    /// Creates a new `TokenBucket` with the specified `capacity` and `refill_rate`.
    ///
    /// * `capacity`: The maximum number of tokens the bucket can hold.
    /// * `refill_rate`: Number of tokens added to the bucket every second.
    ///
    /// # Example
    ///
    /// ```rust
    /// use limitr::bucket::TokenBucket;
    /// let bucket = TokenBucket::new(10, 5); // 10 tokens capacity, 5 tokens per second refill rate
    /// ```
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        trace!(
            "Creating a new TokenBucket with capacity: {} and refill rate: {}",
            capacity,
            refill_rate
        );
        Self {
            capacity,
            tokens: capacity, // Start with a full bucket
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Refills the bucket based on the elapsed time since the last refill.
    ///
    /// Adds tokens to the bucket based on the `refill_rate` and the amount of
    /// time that has passed since the last refill. It ensures the bucket does
    /// not exceed the defined `capacity`.
    ///
    /// This function runs synchronously, but is called asynchronously in the context of `try_consume`.
    async fn refill(&mut self) {
        let now = Instant::now();
        let time_since_last_refill = now.duration_since(self.last_refill).as_secs();

        if time_since_last_refill > 0 {
            let tokens_to_add = time_since_last_refill * self.refill_rate;
            trace!(
                "Refilling bucket: adding {} tokens after {} seconds",
                tokens_to_add,
                time_since_last_refill
            );

            self.tokens = (self.tokens + tokens_to_add).min(self.capacity);
            self.last_refill = now;
        } else {
            trace!("No need to refill, less than 1 second has passed.");
        }
    }

    /// Attempts to consume the specified `amount` of tokens asynchronously.
    ///
    /// Refills tokens if necessary before consumption. If there are enough tokens, the request succeeds,
    /// otherwise it fails. Provides detailed tracing of the token state for debugging.
    ///
    /// # Returns
    ///
    /// `true` if tokens were successfully consumed, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use limitr::bucket::TokenBucket;
    /// # tokio_test::block_on(async {
    /// let mut bucket = TokenBucket::new(10, 5);
    /// if bucket.try_consume(2).await {
    ///     println!("Token consumed!");
    /// }
    /// # })
    /// ```
    pub async fn try_consume(&mut self, amount: u64) -> bool {
        self.refill().await;

        if self.tokens >= amount {
            self.tokens -= amount;
            trace!(
                "Consumed {} tokens, {} tokens left in the bucket.",
                amount,
                self.tokens
            );
            true
        } else {
            trace!(
                "Failed to consume {} tokens. Only {} tokens left in the bucket.",
                amount,
                self.tokens
            );
            false
        }
    }

    /// Returns the current number of tokens available in the bucket.
    ///
    /// This is useful for monitoring or logging the current token state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use limitr::bucket::TokenBucket;
    /// # tokio_test::block_on(async {
    /// let mut bucket = TokenBucket::new(10, 5);
    /// println!("Available tokens: {}", bucket.available_tokens().await);
    /// # })
    /// ```
    pub async fn available_tokens(&self) -> u64 {
        self.tokens
    }
}
