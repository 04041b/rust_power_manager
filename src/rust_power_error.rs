

    pub fn craft_error_window_win(message : String, error_title : &str) {
    let title:Vec<u16> = ("rust power manager ERROR: ".to_owned() + error_title + "\0").encode_utf16().collect();
    let winmessage : Vec<u16> = (message+"\0").encode_utf16().collect();

        unsafe {winapi::um::winuser::MessageBoxW(std::ptr::null_mut(), winmessage.as_ptr(),title.as_ptr(),winapi::um::winuser::MB_ICONSTOP); }
    }
