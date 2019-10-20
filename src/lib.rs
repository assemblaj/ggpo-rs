#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::mem;
    use std::os::raw::*;

    // Stub callback functions
    unsafe extern "C" fn begin_game_callback(game: *const c_char) -> bool {
        true
    }

    unsafe extern "C" fn advance_frame_callback(flags: c_int) -> bool {
        true
    }

    unsafe extern "C" fn load_game_state_callback(buffer: *mut c_uchar, len: c_int) -> bool {
        true
    }

    unsafe extern "C" fn save_game_state_callback(
        buffer: *mut *mut c_uchar,
        len: *mut c_int,
        checksum: *mut c_int,
        frame: c_int,
    ) -> bool {
        true
    }

    unsafe extern "C" fn free_buffer_callback(buffer: *mut c_void) {}

    unsafe extern "C" fn on_event_callback(info: *mut GGPOEvent) -> bool {
        true
    }

    #[test]
    /*
        Sanity test for generated bindings. Tests that all examples provided in
        the GGPO development guide works correctly.
        Link to guide: https://github.com/pond3r/ggpo/blob/master/doc/DevelopmentGuide.md
    */
    fn ggpo_basic_test() {
        unsafe {
            /*
                Creating the GGPOSession Object
            */
            let mut ggpo_ptr: *mut GGPOSession = std::ptr::null_mut();
            let mut ggpo = GGPOSession { _unused: [0; 0] };
            ggpo_ptr = &mut ggpo;

            let cb_ptr: *mut GGPOSessionCallbacks;
            let mut cb = GGPOSessionCallbacks {
                begin_game: Some(begin_game_callback),
                advance_frame: Some(advance_frame_callback),
                load_game_state: Some(load_game_state_callback),
                save_game_state: Some(save_game_state_callback),
                free_buffer: Some(free_buffer_callback),
                on_event: Some(on_event_callback),
                log_game_state: None, // not required by GGPO
            };
            cb_ptr = &mut cb;

            let mut result: GGPOErrorCode;
            let name_str = CString::new("Test App").expect("CString::new failed");

            result = ggpo_start_session(
                &mut ggpo_ptr,                // the new session object
                cb_ptr,                       // our callbacks
                name_str.as_ptr(),            // application name
                2,                            // 2 players
                mem::size_of::<i32>() as i32, // size of an input packet
                8001,                         // our local udp port
            );

            assert_eq!(result, GGPOErrorCode_GGPO_OK);

            result = ggpo_close_session(ggpo_ptr);

            assert_eq!(result, GGPOErrorCode_GGPO_OK);
        }
    }
}
