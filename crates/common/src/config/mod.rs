use zenoh::Config;

const DEFAULT_CONFIG_JSON5: &str = include_str!("zenoh_config.json5");

pub fn get_default() -> Config {
    return Config::from_json5(DEFAULT_CONFIG_JSON5).unwrap();
}
