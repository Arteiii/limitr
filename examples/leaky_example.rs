//! Example of using the `LeakyBucket` rate limiter with randomized request intervals and durations

use limitr::bucket::LeakyBucket;
use rand::Rng;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let mut bucket = LeakyBucket::new(10, 2); // Capacity of 10, leak rate of 2 tokens per second
    let mut rng = rand::thread_rng();

    for i in 0..60 {
        let sleep_duration = rng.gen_range(10..=400);
        if bucket.try_consume().await {
            println!("Leaky Bucket Example: Request {} succeeded.", i + 1);
        } else {
            println!(
                "Leaky Bucket Example: Request {} failed, bucket is empty.",
                i + 1
            );
        }
        sleep(Duration::from_millis(sleep_duration)).await;
    }
}
