#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)] 
mod tests {
    use super::*;
    use std::mem;
    use std::ffi::CString;
    use std::os::raw::*;

    // Stub callback functions 
    unsafe extern "C" fn begin_game_callback (game: *const c_char) -> bool {
        return true; 
    }
    
    unsafe extern "C" fn advance_frame_callback(flags: c_int) -> bool {
        return true; 
    }

    unsafe extern "C" fn load_game_state_callback(buffer: *mut c_uchar, len: c_int) -> bool {
        return true;
    }

    unsafe extern "C" fn save_game_state_callback(buffer: *mut *mut c_uchar, len: *mut c_int, checksum: *mut c_int, frame: c_int) -> bool {
        return true;
    }
    
    unsafe extern "C" fn free_buffer_callback(buffer: *mut c_void) {}

    unsafe extern "C" fn on_event_callback(info: *mut GGPOEvent) -> bool {
        return true;
    }

    #[test]
    fn ggpo_basic_test() {
        unsafe {
            let mut ggpo: *mut GGPOSession = std::ptr::null_mut();
            let result: GGPOErrorCode;
            let cb: *mut GGPOSessionCallbacks = std::ptr::null_mut();
            let name_str = CString::new("Test App").expect("CString::new failed");
                        
            /* fill in all callback functions */
            (*cb).begin_game = Some(begin_game_callback); 
            (*cb).advance_frame = Some(advance_frame_callback); 
            (*cb).load_game_state = Some(load_game_state_callback); 
            (*cb).save_game_state = Some(save_game_state_callback); 
            (*cb).free_buffer = Some(free_buffer_callback); 
            (*cb).on_event = Some(on_event_callback); 

            /* Start a new session */
            result = ggpo_start_session(&mut ggpo,                    // the new session object
                                        cb,                           // our callbacks
                                        name_str.as_ptr(),            // application name
                                        2,                            // 2 players
                                        mem::size_of::<i32>() as i32, // size of an input packet
                                        8001);                        // our local udp port

            assert_eq!(result, GGPOErrorCode_GGPO_OK);
        }
    }
}




