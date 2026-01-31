use tokio::time;
use utils::defaults::{DEFAULT_LOOP_SLEEP, DEFAULT_WAIT_SLEEP};

#[tokio::main]
async fn main() {
    let session = utils::session::get_default().await;
    let publisher = session.declare_publisher("example/value").await.unwrap();
    time::sleep(DEFAULT_WAIT_SLEEP).await;
    for i in 1..=3 {
        let value = format!("hello#{}", i);
        println!("Publishing at {}: {}", publisher.key_expr(), value);
        publisher.put(value).await.unwrap();
        time::sleep(DEFAULT_LOOP_SLEEP).await;
    }
}
