//! Example of using the `TokenBucket` rate limiter with randomized token requirements.

use limitr::bucket::TokenBucket;
use rand::Rng;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Capacity of 10, refill rate of 2 tokens per second
    let mut bucket = TokenBucket::new(20, 2);
    let mut rng = rand::thread_rng();

    for i in 0..60 {
        let tokens_required = rng.gen_range(1..=6);

        if bucket.try_consume(tokens_required).await {
            println!(
                "Token Bucket Example: Request {} ({} tokens) succeeded.",
                i + 1,
                tokens_required
            );
        } else {
            println!(
                "Token Bucket Example: Request {} ({} tokens) failed, not enough tokens.",
                i + 1,
                tokens_required
            );
        }

        let sleep_duration = rng.gen_range(100..=1000);
        sleep(Duration::from_millis(sleep_duration)).await;
    }
}
