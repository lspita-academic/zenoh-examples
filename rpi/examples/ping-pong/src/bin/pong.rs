use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting pong task");
    let config = zenoh::Config::from_json5(ping_pong::CONFIG_JSON)
    .unwrap();
    let session = zenoh::open(config).await.unwrap();
    let publisher = session.declare_publisher("pong/value").await.unwrap();
    let subscriber = session.declare_subscriber("ping/value").await.unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;
    let peers = session.info().peers_zid().await.collect::<Vec<_>>();
    println!("Peers: {:?}", peers);
    let mut count = 0;
    loop {
        let pong = count.to_string();
        println!("Waiting for value on {}", subscriber.key_expr());
        let sample = subscriber.recv_async().await.unwrap();
        let ping = sample.payload().try_to_string().unwrap();
        println!("Received ping: {}", ping);
        println!("Publishing on {}: {}", publisher.key_expr(), pong);
        tokio::time::sleep(Duration::from_millis(2000)).await;
        publisher.put(&pong).await.unwrap();
        count += 1;
    }
    // session.close().await.unwrap();
}
