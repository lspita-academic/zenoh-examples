use embassy_executor::Spawner;
use embassy_time::Timer;
use esp_idf_svc::log::EspLogger;

#[embassy_executor::task]
async fn hello_world() {
    loop {
        log::info!("Hello, world!");
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
    let _ = spawner.spawn(hello_world());
}
