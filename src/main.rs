use std::ffi::CStr;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::zenoh_pico::{
    Z_CONFIG_MODE_KEY, z_config_default, z_config_drop, z_config_loan, z_config_loan_mut,
    z_config_move, z_owned_config_t, zp_config_get, zp_config_insert,
};
use static_cell::StaticCell;

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
    pub fn get(&self, key: u32) -> Option<&str> {
        unsafe {
            let ptr = zp_config_get(z_config_loan(&self.config), key as u8);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().expect(&format!(
                    "Zenoh config key {} is not a valid UTF-8 string",
                    key
                )))
            }
        }
    }

    pub fn set(&mut self, key: u32, value: &str) -> Result<&str, i8> {
        let result = unsafe {
            let value_bytes = [value.as_bytes(), &[0]].concat();
            let value_cstr = CStr::from_bytes_until_nul(value_bytes.as_slice()).unwrap();
            zp_config_insert(
                z_config_loan_mut(&mut self.config),
                key as u8,
                value_cstr.as_ptr(),
            )
        };
        if result == 0 {
            let value = self.get(key).unwrap();
            Ok(value)
        } else {
            Err(result)
        }
    }
}

static ZENOH_CONFIG: StaticCell<ZenohConfig> = StaticCell::new();

#[embassy_executor::task]
async fn hello_world(zenoh_config: &'static ZenohConfig) {
    let config_mode = zenoh_config
        .get(Z_CONFIG_MODE_KEY)
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

    zenoh_config
        .set(Z_CONFIG_MODE_KEY, "peer")
        .expect("Failed to set zenoh config to peer mode");
    let _ = spawner.spawn(hello_world(zenoh_config));
}
