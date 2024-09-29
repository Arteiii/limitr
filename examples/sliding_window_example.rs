use tokio::time::Duration;
use limitr::window::SlidingWindowCounter;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Create a rate limiter that allows 5 requests per 10-second window
    let limiter = Arc::new(Mutex::new(SlidingWindowCounter::new(5, Duration::from_secs(10))));

    // Simulate multiple requests being made
    for i in 1..=10 {
        let limiter_clone = Arc::clone(&limiter);
        let result = tokio::spawn(async move {
            let mut limiter = limiter_clone.lock().await;
            if limiter.try_consume().await {
                println!("Request {} allowed", i);
            } else {
                println!("Request {} rate-limited", i);
            }
        });

        result.await.unwrap();
    }

    // Add a delay to see requests outside the window
    tokio::time::sleep(Duration::from_secs(11)).await;

    // Try again after the window has expired
    let mut limiter = limiter.lock().await;
    if limiter.try_consume().await {
        println!("New request allowed after window expiration");
    } else {
        println!("New request rate-limited after window expiration");
    }
}
