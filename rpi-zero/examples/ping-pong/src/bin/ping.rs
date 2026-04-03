use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting ping task");
    let config = zenoh::Config::from_json5(rpi_zero_ping_pong::CONFIG_JSON)
    .unwrap();
    let session = zenoh::open(config).await.unwrap();
    let publisher = session.declare_publisher("ping/value").await.unwrap();
    let subscriber = session.declare_subscriber("pong/value").await.unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;
    let peers = session.info().peers_zid().await.collect::<Vec<_>>();
    println!("Peers: {:?}", peers);
    let mut count = 0;
    loop {
        let ping = count.to_string();
        tokio::time::sleep(Duration::from_millis(2000)).await;
        println!("Publishing on {}: {}", publisher.key_expr(), ping);
        publisher.put(&ping).await.unwrap();
        println!("Waiting for value on {}", subscriber.key_expr());
        let sample = subscriber.recv_async().await.unwrap();
        let pong = sample.payload().try_to_string().unwrap();
        println!("Received pong: {}", pong);
        assert_eq!(pong, ping);
        count += 1;
    }
    // session.close().await.unwrap();
}
