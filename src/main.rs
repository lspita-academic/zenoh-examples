mod wifi;
mod zenoh;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use static_cell::StaticCell;
use zenoh::config::{ZenohConfig, ZenohConfigKey};

static ZENOH_CONFIG: StaticCell<ZenohConfig> = StaticCell::new();

#[embassy_executor::task]
async fn hello_world(zenoh_config: &'static ZenohConfig) {
    let config_mode = zenoh_config
        .get(ZenohConfigKey::Mode)
        .expect("Zenoh config mode key not found")
        .expect("Zenoh config mode key is not a valid UTF-8 string");

    loop {
        log::info!("Hello, {}", config_mode);
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
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

    let zenoh_config = ZENOH_CONFIG.init(Default::default());

    zenoh_config
        .set(ZenohConfigKey::Mode, "peer")
        .expect("Failed to set zenoh config to peer mode");
    let _ = spawner.spawn(hello_world(zenoh_config));
}
