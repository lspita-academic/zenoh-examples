use zenoh::Session;

use crate::config;

pub async fn get_default() -> Session {
    let config = config::get_default();
    return zenoh::open(config).await.unwrap();
}
