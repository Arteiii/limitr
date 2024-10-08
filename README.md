# limitr

[![codecov](https://codecov.io/gh/Arteiii/limitr/graph/badge.svg?token=DKD1ZYRT5D)](https://codecov.io/gh/Arteiii/limitr)
[![Check and Lint](https://github.com/Arteiii/limitr/actions/workflows/check_and_lint.yml/badge.svg)](https://github.com/Arteiii/limitr/actions/workflows/check_and_lint.yml)
[![CodeFactor](https://www.codefactor.io/repository/github/arteiii/limitr/badge)](https://www.codefactor.io/repository/github/arteiii/limitr)

![Crates.io Version](https://img.shields.io/crates/v/limitr)

`limitr` is a Rust crate that provides implementations of rate-limiting algorithms for controlling the rate of requests
or operations. It includes various algorithms such as Token Bucket, Leaky Bucket, Sliding Window, and Fixed Window,
which are commonly used to manage and limit request rates in applications.

## Features

- **Token Bucket**: Allows requests to be processed at a burst rate up to a certain capacity and then at a steady rate.
- **Leaky Bucket**: Ensures a steady rate of processing by "leaking" requests at a constant rate, regardless of incoming
  request burstiness.
- **Sliding Window**: Provides a more accurate request limiting mechanism by keeping track of individual requests over a
  moving window of time.
- **Fixed Window**: Counts requests in fixed intervals, simpler than Sliding Window but can lead to bursts at the
  boundary of two windows.

## Installation

Add `limitr` to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1.40.0", features = ["full"] }
limitr = "0.1.0"
```

## Usage

### Token Bucket

```rust
use limitr::bucket::TokenBucket;
use tokio::time::{sleep, Duration};
use rand::Rng;

#[tokio::main]
async fn main() {
    let mut bucket = TokenBucket::new(10, 2); // Capacity of 10, refill rate of 2 tokens per second
    let mut rng = rand::thread_rng();

    let total_requests = rng.gen_range(10..=20);

    for i in 0..total_requests {
        let tokens_required = rng.gen_range(1..=3);
        if bucket.try_consume(tokens_required).await {
            println!("Token Bucket Example: Request {} ({} tokens) succeeded.", i + 1, tokens_required);
        } else {
            println!("Token Bucket Example: Request {} ({} tokens) failed, not enough tokens.", i + 1, tokens_required);
        }
        let sleep_duration = rng.gen_range(100..=1000);
        sleep(Duration::from_millis(sleep_duration)).await;
    }
}
```

### Leaky Bucket

```rust
use limitr::bucket::LeakyBucket;
use tokio::time::{sleep, Duration};
use rand::Rng;

#[tokio::main]
async fn main() {
    let mut bucket = LeakyBucket::new(10, 2); // Capacity of 10, leak rate of 2 tokens per second
    let mut rng = rand::thread_rng();

    let total_requests = rng.gen_range(10..=20);

    for i in 0..total_requests {
        if bucket.try_consume().await {
            println!("Leaky Bucket Example: Request {} succeeded.", i + 1);
        } else {
            println!("Leaky Bucket Example: Request {} failed, bucket is empty.", i + 1);
        }
        let sleep_duration = rng.gen_range(100..=1000);
        sleep(Duration::from_millis(sleep_duration)).await;
    }
}
```

### Sliding Window

```rust
use limitr::window::SlidingWindowCounter;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mut limiter = SlidingWindowCounter::new(5, Duration::from_secs(10)); // Allow 5 requests per 10-second window

    for _ in 0..6 {
        if limiter.try_consume().await {
            println!("Sliding Window Example: Request succeeded.");
        } else {
            println!("Sliding Window Example: Request rate-limited.");
        }
    }
}
```

### Fixed Window

```rust
use limitr::window::FixedWindowCounter;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let mut limiter = FixedWindowCounter::new(5, Duration::from_secs(10)); // Allow 5 requests per 10-second window

    let now = Instant::now();
    for _ in 0..6 {
        if limiter.try_consume(now) {
            println!("Fixed Window Example: Request succeeded.");
        } else {
            println!("Fixed Window Example: Request rate-limited.");
        }
    }
}
```

## Features

The crate includes the following features:

- `bucket` (default): Enables the Token Bucket and Leaky Bucket implementations.
- `window`: Enables the Sliding Window and Fixed Window implementations.
- `full`: Includes additional features or configurations if needed.

To enable specific features, use:

```toml
[dependencies.limitr]
features = ["feature_name"]
```

## License

`limitr` is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request to contribute to the project.
