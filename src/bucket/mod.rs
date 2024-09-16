//! Bucket Algorithms Module
//!
//! This module provides various implementations of rate-limiting algorithms based on the bucket concept.
//! These algorithms control the rate at which requests are processed, making them useful for managing
//! traffic and preventing overloads in systems with varying load patterns.
//!
//! ## Available Algorithms
//!
//! - **Token Bucket**: This algorithm allows a burst of requests up to a specified limit, and then
//!   processes requests at a steady rate. It can accommodate sudden bursts of traffic but will
//!   throttle the rate if the burst capacity is exceeded.
//!
//! - **Leaky Bucket**: This algorithm ensures a steady rate of processing requests, leaking them at
//!   a constant rate. It smooths out burstiness in traffic and maintains a consistent processing rate,
//!   dropping requests if the bucket is full.
//!
//! ## Usage
//!
//! To use these algorithms, you need to create an instance of the desired bucket type and configure it
//! with appropriate parameters such as capacity and leak rate. You can then use the `try_consume` method
//! to attempt to process a request, which will either succeed or fail depending on the bucket's state.
//!
//! ```rust
//! use limitr::bucket::{LeakyBucket, TokenBucket};
//! use tokio::time::Duration;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut leaky_bucket = LeakyBucket::new(10, 2);
//!     let mut token_bucket = TokenBucket::new(10, 5);
//!
//!     for i in 0..15 {
//!         if leaky_bucket.try_consume().await {
//!             println!("Leaky Bucket: Request {} succeeded.", i + 1);
//!         } else {
//!             println!("Leaky Bucket: Request {} failed, bucket is empty.", i);
//!         }
//!
//!         if token_bucket.try_consume(1).await {
//!             println!("Token Bucket: Request {} succeeded.", i + 1);
//!         } else {
//!             println!("Token Bucket: Request {} failed, bucket is empty.", i);
//!         }
//!
//!         tokio::time::sleep(Duration::from_millis(500)).await;
//!     }
//! }
//! ```

pub mod leaky;
pub mod token;

pub use leaky::*;
pub use token::*;
