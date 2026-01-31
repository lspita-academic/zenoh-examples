use std::time::Duration;
use tokio::time;

use hello_zenoh::KEY_PUB_SUB;

#[tokio::main]
async fn main() {
    println!("Starting publisher");

    let config = config::get_default();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let publisher = session.declare_publisher(KEY_PUB_SUB).await.unwrap();
    println!("Publisher for {} created successfully", KEY_PUB_SUB);

    println!("Starting publishing");
    for i in 0..10 {
        let buffer = format!("HELLO#{}", i);
        println!("Publishing {} at {}", buffer, KEY_PUB_SUB);
        publisher.put(buffer).await.unwrap();
        time::sleep(Duration::from_secs(1)).await;
    }
    println!("Finished publishing");
}
