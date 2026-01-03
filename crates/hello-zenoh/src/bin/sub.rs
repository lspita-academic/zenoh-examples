use hello_zenoh::{KEY, get_config};

#[tokio::main]
async fn main() {
    println!("Starting subscriber");
    let config = get_config();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");
    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());
    println!(
        "Routers: {:?}",
        info.routers_zid().await.collect::<Vec<_>>()
    );

    let subscriber = session.declare_subscriber(KEY).await.unwrap();
    println!("Subscriber for {} created successfully", KEY);
    println!("Starting read loop");
    while let Ok(sample) = subscriber.recv_async().await {
        let payload = sample
            .payload()
            .try_to_string()
            .expect("payload must be a string");
        println!("Received {} from {}", payload, sample.key_expr());
    }
    println!("Finished reading");
}
