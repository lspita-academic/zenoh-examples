use routers::KEY;
use utils::defaults::DEFAULT_RECV_TIMEOUT;

#[tokio::main]
async fn main() {
    let session = utils::session::get_default().await;
    let subscriber = session.declare_subscriber(KEY).await.unwrap();

    let key = subscriber.key_expr();
    println!("Starting receiving at {}", key);
    while let Ok(Some(sample)) = subscriber.recv_timeout(DEFAULT_RECV_TIMEOUT) {
        let value = sample.payload().try_to_string().unwrap();
        println!("Received at {}: {}", key, &value);
    }
    println!("Finished receiving at {}", key);
}
