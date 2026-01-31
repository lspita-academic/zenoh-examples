use key_expressions::run_subscriber;

#[tokio::main]
async fn main() {
    let session = utils::session::get_default().await;
    let subscriber = session.declare_subscriber("example/value").await.unwrap();
    println!("Subscriber {} started", subscriber.key_expr());
    run_subscriber(&subscriber).await;
    println!("Closing subscriber {}", subscriber.key_expr());
}
