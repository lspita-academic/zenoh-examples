use routers::KEY;
use tokio::time;
use utils::defaults::{DEFAULT_LOOP_SLEEP, DEFAULT_WAIT_SLEEP};

#[tokio::main]
async fn main() {
    let session = utils::session::get_default().await;
    let publisher = session.declare_publisher(KEY).await.unwrap();

    time::sleep(DEFAULT_WAIT_SLEEP).await;
    let key = publisher.key_expr();
    println!("Starting publishing at {}", key);
    for i in 1..=5 {
        let peers = session.info().peers_zid().await.collect::<Vec<_>>();
        println!("Peers: {:?}", peers);
        let value = format!("HELLO#{}", i);
        publisher.put(&value).await.unwrap();
        println!("Published at {}: {}", key, &value);
        time::sleep(DEFAULT_LOOP_SLEEP).await;
    }
    println!("Finished publishing at {}", key);
}
