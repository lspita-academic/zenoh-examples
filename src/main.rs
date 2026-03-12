use static_cell::StaticCell;
use std::ffi::CStr;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::zenoh_pico::{
    Z_CONFIG_MODE_KEY, z_config_default, z_config_drop, z_config_loan, z_config_move,
    z_owned_config_t, z_random_u8, zp_config_get,
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

unsafe impl Send for ZenohConfig {}
unsafe impl Sync for ZenohConfig {} // safe because tasks only read after init

static ZENOH_CONFIG: StaticCell<ZenohConfig> = StaticCell::new();

#[embassy_executor::task]
async fn hello_world(zenoh_config: &'static ZenohConfig) {
    let config_mode_ptr =
        unsafe { zp_config_get(z_config_loan(&zenoh_config.config), Z_CONFIG_MODE_KEY as u8) };
    if config_mode_ptr == 0x0 as *const u8 {
        panic!("Config mode not found!")
    }
    let config_mode = unsafe { CStr::from_ptr(config_mode_ptr) };
    log::info!("Config mode: {:?}", config_mode);

    loop {
        let random_n;
        unsafe {
            // Example: call zenoh-pico function
            random_n = z_random_u8();
        }
        log::info!("Hello, world: {}", random_n);
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
