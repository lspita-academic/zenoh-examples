use std::time::Duration;

use hello_zenoh::KEY;

#[tokio::main]
async fn main() {
    println!("Starting subscriber");

    let config = config::get_default();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let subscriber = session.declare_subscriber(KEY).await.unwrap();
    println!("Subscriber for {} created successfully", KEY);

    println!("Starting read loop");
    let timeout = Duration::from_secs(3);
    loop {
        match subscriber.recv_timeout(timeout) {
            Err(e) => {
                println!("Error receiving data: {:?}", e);
                break;
            }
            Ok(None) => {
                println!("Time exceeded to receive data");
                break;
            }
            Ok(Some(sample)) => {
                let payload = sample
                    .payload()
                    .try_to_string()
                    .expect("payload must be a string");
                println!("Received {} from {}", payload, sample.key_expr());
            }
        }
    }
    println!("Finished reading");
}
