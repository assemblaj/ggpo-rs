#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)] 
mod tests {
    use super::*;
    use std::mem;
    use std::ffi::CString;

    #[test]
    fn ggpo_basic_test() {
        unsafe {
            let mut ggpo: *mut GGPOSession = std::ptr::null_mut();
            let result: GGPOErrorCode;
            let cb: *mut GGPOSessionCallbacks = std::ptr::null_mut();
            let name_str = CString::new("Test App").expect("CString::new failed");

            
            /* fill in all callback functions */
            // cb.begin_game = vw_begin_game_callback;
            // cb.advance_frame = vw_advance_frame_callback;
            // cb.load_game_state = vw_load_game_state_callback;
            // cb.save_game_state = vw_save_game_state_callback;
            // cb.free_buffer = vw_free_buffer;
            // cb.on_event = vw_on_event_callback;

            /* Start a new session */
            result = ggpo_start_session(&mut ggpo,         // the new session object
                                        cb,           // our callbacks
                                        name_str.as_ptr(),    // application name
                                        2,             // 2 players
                                        mem::size_of::<i32>() as i32,   // size of an input packet
                                        8001);         // our local udp port

            assert_eq!(result, GGPOErrorCode_GGPO_OK);
        }
    }
}




