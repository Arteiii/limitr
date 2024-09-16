//! Limitr: Rate-Limiting Algorithms Library
//!
//! This library provides various algorithms for rate-limiting purposes, allowing control over
//! the flow of requests or operations in a system. The algorithms in this crate include:
//!
//! - **Token Bucket**: For burstable traffic control, where tokens accumulate over time and are consumed by requests.
//! - **Leaky Bucket**: For smoothing out traffic, where requests are allowed to "leak" out at a fixed rate.
//!
//! ## Example Usage
//!
//! ```rust
//! use limitr::bucket::LeakyBucket;
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut bucket = LeakyBucket::new(10, 2);  // Capacity of 10, leak rate of 2 per second
//!
//!     for i in 0..15 {
//!         if bucket.try_consume().await {
//!             println!("Request {} succeeded.", i + 1);
//!         } else {
//!             println!("Request {} failed, bucket is empty.", i);
//!         }
//!         sleep(Duration::from_millis(500)).await;
//!     }
//! }
//! ```

#[cfg(feature = "bucket")]
pub mod bucket;
