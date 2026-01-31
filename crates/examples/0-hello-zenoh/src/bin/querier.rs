use std::time::Duration;
use tokio::time;

use hello_zenoh::KEY_QUERY_REPLY;
use zenoh::Config;

#[tokio::main]
async fn main() {
    println!("Starting querier");

    let config = Config::default();
    let session = zenoh::open(config).await.unwrap();
    println!("Session opened successfully");

    let info = session.info();
    println!("Session ID: {}", info.zid().await);
    println!("Peers: {:?}", info.peers_zid().await.collect::<Vec<_>>());

    let querier = session.declare_querier(KEY_QUERY_REPLY).await.unwrap();
    println!("Querier for {} created successfully", KEY_QUERY_REPLY);

    let wait_duration = Duration::from_secs(2);
    println!(
        "Waiting {} seconds for queriable to start",
        wait_duration.as_secs()
    );
    time::sleep(wait_duration).await;
    println!("Starting querying");
    let timeout = Duration::from_secs(3);
    loop {
        println!("Querying at {}", querier.key_expr());
        match querier.get().await {
            Err(e) => {
                // Disconnection of queriable is considered an error.
                // In a real application, the queriable is always online, and should be interrogated when needed.
                // In this demo, we query until disconnected
                println!("Error receiving data: {:?}", e);
                break;
            }
            Ok(replies) => {
                match replies.recv_timeout(timeout) {
                    Err(e) => {
                        println!("Error receiving reply: {:?}", e);
                        break;
                    }
                    Ok(None) => {
                        println!("Time exceeded to receive data");
                        break;
                    }
                    Ok(Some(reply)) => {
                        let sample = reply.result().unwrap();
                        let payload = sample
                            .payload()
                            .try_to_string()
                            .expect("payload must be a string");
                        println!("Received {} from {}", payload, sample.key_expr());
                    }
                }
                time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
    println!("Finished querying");
}
