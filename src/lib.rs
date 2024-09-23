//! Limitr: Rate-Limiting Algorithms Library
//!
//! [![codecov](https://codecov.io/gh/Arteiii/limitr/graph/badge.svg?token=DKD1ZYRT5D)](https://codecov.io/gh/Arteiii/limitr)
//! [![Check and Lint](https://github.com/Arteiii/limitr/actions/workflows/check_and_lint.yml/badge.svg)](https://github.com/Arteiii/limitr/actions/workflows/check_and_lint.yml)
//! 
//! [![GitHub]](https://github.com/Arteiii/limitr)&ensp;[![docs-rs]](https://docs.rs/limitr/latest/limitr/)&ensp;[![crates-io]](https://crates.io/crates/limitr/)
//!
//! [GitHub]:
//! https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]:
//! https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]:
//! https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
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

#[cfg(feature = "window")]
pub mod window;
