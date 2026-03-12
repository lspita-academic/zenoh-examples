use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use static_cell::StaticCell;

mod zenoh;

use zenoh::ZenohConfig;

use crate::zenoh::ZenohConfigKey;

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
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    let zenoh_config = ZENOH_CONFIG.init(Default::default());

    zenoh_config
        .set(ZenohConfigKey::Mode, "peer")
        .expect("Failed to set zenoh config to peer mode");
    let _ = spawner.spawn(hello_world(zenoh_config));
}
