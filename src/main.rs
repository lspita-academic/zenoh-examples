use std::ffi::CStr;

use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys::zenoh_pico;

#[embassy_executor::task]
async fn hello_world() {
    loop {
        let random_n;
        unsafe {
            // Example: call zenoh-pico function
            random_n = zenoh_pico::z_random_u8();
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

    let mut zenoh_config = zenoh_pico::z_owned_config_t::default();
    unsafe {
        let ok = zenoh_pico::z_config_default(&mut zenoh_config);
        assert!(ok == 0, "Cannot create default zenoh config");
        let config_mode_ptr = zenoh_pico::zp_config_get(
            zenoh_pico::z_config_loan(&zenoh_config),
            zenoh_pico::Z_CONFIG_MODE_KEY as u8,
        );
        if config_mode_ptr == 0x0 as *const u8 {
            panic!("Config mode not found!")
        }
        let config_mode = CStr::from_ptr(config_mode_ptr);
        log::info!("Config mode: {:?}", config_mode);
    }

    let _ = spawner.spawn(hello_world());
}
