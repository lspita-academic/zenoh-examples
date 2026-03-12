use esp_idf_svc::sys::zenoh_pico::{
    _z_res_t_Z_OK, z_close, z_config_move, z_open, z_open_options_t, z_owned_session_t,
    z_session_drop, z_session_is_closed, z_session_loan, z_session_loan_mut, z_session_move,
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
}
