use std::time::Duration;

use hello_zenoh::KEY_QUERY_REPLY;

#[tokio::main]
async fn main() {
    println!("Starting queryable");

    let config = config::get_default();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let queryable = session.declare_queryable(KEY_QUERY_REPLY).await.unwrap();
    println!("Queryable for {} created successfully", KEY_QUERY_REPLY);

    println!("Starting reply loop");
    let timeout = Duration::from_secs(3);
    for i in 0..10 {
        match queryable.recv_timeout(timeout) {
            Err(e) => {
                println!("Error receiving data: {:?}", e);
                break;
            }
            Ok(None) => {
                println!("Time exceeded to receive query");
                break;
            }
            Ok(Some(query)) => {
                let buffer = format!("HELLO#{}", i);
                println!("Replying with {} at {}", buffer, query.selector());
                query.reply(KEY_QUERY_REPLY, buffer).await.unwrap();
            }
        }
    }
    println!("Finished replying");
}
