use utils::defaults::DEFAULT_RECV_TIMEOUT;
use zenoh::{handlers::FifoChannelHandler, pubsub::Subscriber, sample::Sample};

pub async fn run_subscriber(subscriber: &Subscriber<FifoChannelHandler<Sample>>) {
    while let Ok(Some(sample)) = subscriber.recv_timeout(DEFAULT_RECV_TIMEOUT) {
        let payload = sample.payload().try_to_string().unwrap();
        println!("Received from {}: {}", sample.key_expr(), payload);
    }
}
