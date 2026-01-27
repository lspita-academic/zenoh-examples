use std::time::Duration;
use tokio::time;

use hello_zenoh_pub_sub::{KEY, get_config};

#[tokio::main]
async fn main() {
    println!("Starting publisher");

    let config = get_config();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let publisher = session.declare_publisher(KEY).await.unwrap();
    println!("Publisher for {} created successfully", KEY);

    println!("Starting publishing");
    for i in 0..10 {
        let buffer = format!("HELLO#{}", i);
        println!("Publishing {} at {}", buffer, KEY);
        publisher.put(buffer).await.unwrap();
        time::sleep(Duration::from_secs(1)).await;
    }
    println!("Finished publishing");
}
