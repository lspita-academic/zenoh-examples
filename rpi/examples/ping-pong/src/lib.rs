// https://github.com/eclipse-zenoh/zenoh/blob/main/DEFAULT_CONFIG.json5
pub const CONFIG_JSON: &str = concat!(
    r#"
    {
        "mode": "peer",
        "connect": {
            "endpoints": []
        },
        "listen": {
            "endpoints": ["udp/224.0.0.224:7447"]
        },
        "scouting": {
            "timeout": 30000,
            "multicast": {
                "address": "224.0.0.224:7446"
            }
        },
        "transport": {
            "link": {
                "tx": {
                    "batch_size": "#,
    env!("ZENOH_PICO_BATCH_MULTICAST_SIZE"),
    r#"
                }
            }
        }
    }
    "#,
);
