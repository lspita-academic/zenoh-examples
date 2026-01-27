use hello_zenoh_query_reply::{KEY, get_config};

#[tokio::main]
async fn main() {
    println!("Starting subscriber");

    let config = get_config();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let queryable = session.declare_queryable(KEY).await.unwrap();
    println!("Queryable for {} created successfully", KEY);

    println!("Starting reply loop");
    for i in 0..10 {
        match queryable.recv_async().await {
            Err(e) => {
                println!("Error receiving data: {:?}", e);
                break;
            }
            Ok(query) => {
                let buffer = format!("HELLO#{}", i);
                println!("Replying with {} at {}", buffer, query.selector());
                query.reply(KEY, buffer).await.unwrap();
            }
        }
    }
    println!("Finished replying");
}
