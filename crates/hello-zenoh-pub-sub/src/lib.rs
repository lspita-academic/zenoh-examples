use zenoh::Config;

pub const KEY: &str = "hello/world";

pub fn get_config() -> Config {
    // keys reference: https://github.com/eclipse-zenoh/zenoh/blob/main/DEFAULT_CONFIG.json5
    return Config::default();
}
