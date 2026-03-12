use static_cell::StaticCell;
use std::ffi::CStr;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::zenoh_pico::{
    Z_CONFIG_MODE_KEY, z_config_default, z_config_drop, z_config_loan, z_config_move,
    z_owned_config_t, zp_config_get,
};

#[derive(Debug, Default)]
struct ZenohConfig {
    config: z_owned_config_t,
}

impl From<z_owned_config_t> for ZenohConfig {
    fn from(config: z_owned_config_t) -> Self {
        Self { config }
    }
}

impl Drop for ZenohConfig {
    fn drop(&mut self) {
        unsafe {
            z_config_drop(z_config_move(&mut self.config));
        }
    }
}

impl ZenohConfig {
    pub fn get_key(&self, key: u32) -> Option<&str> {
        unsafe {
            let ptr = zp_config_get(z_config_loan(&self.config), key as u8);
            if ptr == 0x0 as *const u8 {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().expect(&format!(
                    "Zenoh config key {} is not a valid UTF-8 string",
                    key
                )))
            }
        }
    }
}

static ZENOH_CONFIG: StaticCell<ZenohConfig> = StaticCell::new();

#[embassy_executor::task]
async fn hello_world(zenoh_config: &'static ZenohConfig) {
    let config_mode = zenoh_config
        .get_key(Z_CONFIG_MODE_KEY)
        .expect("Zenoh config mode key not found");

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

    let zenoh_config = unsafe {
        let mut zenoh_owned_config = Default::default();
        let ok = z_config_default(&mut zenoh_owned_config);
        assert!(ok == 0, "Cannot create default zenoh config");
        let zenoh_config = ZenohConfig::from(zenoh_owned_config);
        ZENOH_CONFIG.init(zenoh_config)
    };

    let _ = spawner.spawn(hello_world(zenoh_config));
}
