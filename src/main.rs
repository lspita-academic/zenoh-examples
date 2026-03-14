mod wifi;
mod zenoh;

use std::time::Duration;

use embassy_executor::Spawner;
use esp_idf_svc::log::EspLogger;
use zenoh::config::{ZenohConfigBuilder, ZenohConfigMode};

use crate::zenoh::session::ZenohSession;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    let mut wifi = wifi::get_wifi().expect("Unable to initialize wifi");
    wifi::connect_wifi(&mut wifi)
        .await
        .unwrap_or_else(|err| panic!("Wifi connection raised error: {:?}", err));

    let ip_info = wifi
        .wifi()
        .sta_netif()
        .get_ip_info()
        .expect("Error getting IP info");
    log::info!("IP address: {}", ip_info.ip);

    let zenoh_config = ZenohConfigBuilder::default()
        .mode(ZenohConfigMode::Peer)
        .scouting_timeout(Duration::from_secs(5))
        .build();

    log::info!("Zenoh config mode: {:?}", zenoh_config.mode());

    ZenohSession::open(zenoh_config, None);
}
