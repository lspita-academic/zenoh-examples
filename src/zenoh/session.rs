use std::ffi::{CStr, c_void};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_idf_svc::sys::zenoh_pico::{
    _z_res_t_Z_OK, _z_sample_t, z_bytes_copy_from_str, z_bytes_move, z_bytes_to_string, z_close, z_closure_sample, z_closure_sample_callback_t, z_closure_sample_drop, z_closure_sample_move, z_config_move, z_declare_subscriber, z_keyexpr_drop, z_keyexpr_from_str, z_keyexpr_loan, z_keyexpr_move, z_open, z_open_options_t, z_owned_bytes_t, z_owned_closure_sample_t, z_owned_keyexpr_t, z_owned_session_t, z_owned_string_t, z_owned_subscriber_t, z_put, z_sample_payload, z_session_drop, z_session_is_closed, z_session_loan, z_session_loan_mut, z_session_move, z_string_data, z_string_len, z_string_loan, z_subscriber_move, z_subscriber_options_default, z_subscriber_options_t, z_undeclare_subscriber
};

use super::config::ZenohConfig;

pub struct ZenohSession {
    pub(super) z_session: z_owned_session_t,
}

impl From<z_owned_session_t> for ZenohSession {
    fn from(z_session: z_owned_session_t) -> Self {
        Self { z_session }
    }
}

impl Drop for ZenohSession {
    fn drop(&mut self) {
        self.close();
        let z_session = &mut self.z_session;
        unsafe {
            z_session_drop(z_session_move(z_session));
        }
    }
}

impl ZenohSession {
    pub fn open(mut config: ZenohConfig, z_open_options: Option<z_open_options_t>) -> Self {
        let z_config = &mut config.z_config;
        let mut z_session = z_owned_session_t::default();
        let open_options = z_open_options
            .map(|o| &o as *const z_open_options_t)
            .unwrap_or(std::ptr::null());

        let result = unsafe { z_open(&mut z_session, z_config_move(z_config), open_options) };
        // TODO: match the result and handle errors. Check _z_res_t enum values.
        assert!(
            result == _z_res_t_Z_OK as i8, // crash if no scouts found
            "Cannot open zenoh session: {}",
            result
        );

        Self::from(z_session)
    }

    pub fn close(&mut self) {
        let z_session = &mut self.z_session;
        let z_session_closed = unsafe { z_session_is_closed(z_session_loan(z_session)) };
        if !z_session_closed {
            let close_options = std::ptr::null();
            unsafe {
                z_close(z_session_loan_mut(z_session), close_options);
            }
        }
    }

    pub fn pub_str(&self, key: &str, value: &str) {
        log::info!("Publishing to {}: {}", key, value);

        let z_session = &self.z_session;
        let options = std::ptr::null();
        let mut key_expr = z_owned_keyexpr_t::default();
        let key_bytes = [key.as_bytes(), &[0]].concat();
        let key_cstr = CStr::from_bytes_until_nul(&key_bytes).unwrap();
        let mut payload_bytes = z_owned_bytes_t::default();
        let value_bytes = [value.as_bytes(), &[0]].concat();
        let value_cstr = CStr::from_bytes_until_nul(&value_bytes).unwrap();
        unsafe {
            z_keyexpr_from_str(&mut key_expr, key_cstr.as_ptr());
            z_bytes_copy_from_str(&mut payload_bytes, value_cstr.as_ptr());
            z_put(
                z_session_loan(z_session),
                z_keyexpr_loan(&key_expr),
                z_bytes_move(&mut payload_bytes),
                options,
            );
            z_keyexpr_drop(z_keyexpr_move(&mut key_expr));
        }
    }

    unsafe extern "C" fn sub_key_callback(sample: *mut _z_sample_t, context: *mut c_void) {
        let signal = unsafe { &mut *(context as *mut Signal<CriticalSectionRawMutex, String>) };
        let payload = unsafe { z_sample_payload(sample) };
        let mut payload_buffer = z_owned_string_t::default();
        let payload_string_bytes;
        let value = unsafe {
            z_bytes_to_string(payload, &mut payload_buffer);
            let payload_z_raw = z_string_data(z_string_loan(&payload_buffer));
            let payload_len = z_string_len(z_string_loan(&payload_buffer));
            payload_string_bytes = [std::slice::from_raw_parts(payload_z_raw, payload_len), &[0]].concat();
            CStr::from_bytes_until_nul(&payload_string_bytes).unwrap().to_str().unwrap()
        };
        signal.signal(value.to_owned());
    }

    pub async fn get_key(&self, key: &str) -> String {
        let z_session = &self.z_session;
        let mut key_expr = z_owned_keyexpr_t::default();
        let key_bytes = [key.as_bytes(), &[0]].concat();
        let key_cstr = CStr::from_bytes_until_nul(&key_bytes).unwrap();

        let mut closure_sample = z_owned_closure_sample_t::default();
        let callback: z_closure_sample_callback_t = Some(Self::sub_key_callback);
        let mut signal = Signal::<CriticalSectionRawMutex, String>::new();

        let mut subscriber = z_owned_subscriber_t::default();
        let mut options = z_subscriber_options_t::default();
        unsafe {
            z_subscriber_options_default(&mut options);
            z_closure_sample(
                &mut closure_sample,
                callback,
                None,
                &mut signal as *mut Signal<CriticalSectionRawMutex, String> as *mut c_void,
            );
            z_keyexpr_from_str(&mut key_expr, key_cstr.as_ptr());
            z_declare_subscriber(
                z_session_loan(z_session),
                &mut subscriber,
                z_keyexpr_loan(&key_expr),
                z_closure_sample_move(&mut closure_sample),
                &options,
            );
        }

        let value = signal.wait().await;
        log::info!("Received from {}: {}", key, value);

        unsafe {
            z_closure_sample_drop(z_closure_sample_move(&mut closure_sample));
            z_keyexpr_drop(z_keyexpr_move(&mut key_expr));
            z_undeclare_subscriber(z_subscriber_move(&mut subscriber));
        }
        value
    }
}
