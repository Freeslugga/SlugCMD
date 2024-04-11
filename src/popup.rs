use std::ptr::null_mut as NULL;
use winapi::um::winuser;

pub struct Popup {
    l_title: Vec<u16>,
    l_content: Vec<u16>,
}

impl Popup {
    pub fn new(t: Vec<u16>, c: Vec<u16>) -> Self {
        Self {
            l_title: t,
            l_content: c,
        }
    }

    pub fn open(&self) {
        if self.l_title.len() > 1 {
            unsafe {
                winuser::MessageBoxW(
                    NULL(),
                    self.l_content.as_ptr(),
                    self.l_title.as_ptr(),
                    winuser::MB_OK | winuser::MB_ICONINFORMATION
                );
            }
        }
    }
}