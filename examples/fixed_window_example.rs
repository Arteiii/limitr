use limitr::window::FixedWindowCounter;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let counter = FixedWindowCounter::new(5, Duration::from_secs(10));

    for _ in 0..5 {
        assert!(counter.try_consume().await, "Request should be allowed");
    }

    println!("All requests in the current window are consumed.");

    // This will be rate-limited because the limit is reached
    assert!(
        !counter.try_consume().await,
        "Request should be rate-limited"
    );

    // Wait for a new window to open
    println!("Waiting for the next time window...");
    tokio::time::sleep(Duration::from_secs(11)).await;

    counter.clear_old_windows().await;

    // Now we should be able to send new requests
    assert!(
        counter.try_consume().await,
        "Request should be allowed in the new window"
    );

    println!("New window allows requests again.");
}
