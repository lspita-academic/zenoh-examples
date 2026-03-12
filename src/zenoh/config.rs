use std::{ffi::CStr, str::Utf8Error};

use esp_idf_svc::sys::zenoh_pico::{
    Z_CONFIG_MODE_KEY, z_config_default, z_config_drop, z_config_loan, z_config_loan_mut,
    z_config_move, z_owned_config_t, zp_config_get, zp_config_insert,
};

pub enum ZenohConfigKey {
    Mode,
}

impl ZenohConfigKey {
    fn z_value(self) -> u8 {
        let key = match self {
            Self::Mode => Z_CONFIG_MODE_KEY,
        };
        key as u8
    }
}

#[derive(Debug)]
pub struct ZenohConfig {
    config: z_owned_config_t,
}

impl From<z_owned_config_t> for ZenohConfig {
    fn from(config: z_owned_config_t) -> Self {
        Self { config }
    }
}

impl Default for ZenohConfig {
    fn default() -> Self {
        let zenoh_owned_config = unsafe {
            let mut zenoh_owned_config = Default::default();
            let ok = z_config_default(&mut zenoh_owned_config);
            assert!(ok == 0, "Cannot create default zenoh config");
            zenoh_owned_config
        };
        Self::from(zenoh_owned_config)
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
    pub fn get(&self, key: ZenohConfigKey) -> Option<Result<&str, Utf8Error>> {
        let value_ptr = unsafe { zp_config_get(z_config_loan(&self.config), key.z_value()) };
        if value_ptr.is_null() {
            None
        } else {
            let value_cstr = unsafe { CStr::from_ptr(value_ptr) };
            Some(value_cstr.to_str())
        }
    }

    pub fn set(&mut self, key: ZenohConfigKey, value: &str) -> Result<(), i8> {
        let value_bytes = [value.as_bytes(), &[0]].concat();
        let value_cstr = CStr::from_bytes_until_nul(value_bytes.as_slice()).unwrap();
        let result = unsafe {
            zp_config_insert(
                z_config_loan_mut(&mut self.config),
                key.z_value(),
                value_cstr.as_ptr(),
            )
        };
        if result == 0 { Ok(()) } else { Err(result) }
    }
}
