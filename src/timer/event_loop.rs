use std::time::Duration;
use tokio::time::{self, Interval};

pub async fn create_loop(sleep_sec: u64) {
    let duration = Duration::from_secs(sleep_sec);
    let mut interval = time::interval(duration);
    loop {
        println!("Hello");
        interval.tick().await;
    }
}
