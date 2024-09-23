//! Window-based rate limiting algorithms.
//!
//! This module provides implementations of window-based rate limiting algorithms:
//!
//! - Fixed Window Counter: Limits requests within fixed time windows.
//! - Sliding Window Counter: Provides a smoother rate limiting approach using a sliding time window.
//!
//! These algorithms are useful for controlling the rate of requests or operations in a system,
//! helping to prevent overload and ensure fair resource usage.
//!
//! # Examples
//!
//! Using the Fixed Window Counter:
//!
//! ```rust
//! use limitr::window::FixedWindowCounter;
//! use std::time::Duration;
//!
//! let mut counter = FixedWindowCounter::new(5, Duration::from_secs(1));
//! # tokio_test::block_on(async {
//! for i in 0..7 {
//!     if counter.try_consume().await {
//!         println!("Request {} allowed", i);
//!     } else {
//!         println!("Request {} denied", i);
//!     }
//! }
//! # })
//! ```
//!
//! Using the Sliding Window Counter:
//!
//! ```rust
//! use limitr::window::SlidingWindowCounter;
//! use std::time::Duration;
//!
//! let mut counter = SlidingWindowCounter::new(5, Duration::from_secs(1));
//! # tokio_test::block_on(async {
//! for i in 0..7 {
//!     if counter.try_consume().await {
//!         println!("Request {} allowed", i);
//!     } else {
//!         println!("Request {} denied", i);
//!     }
//! }
//! # })
//! ```

mod fixed_window;
mod sliding_window;

pub use fixed_window::*;
pub use sliding_window::*;
