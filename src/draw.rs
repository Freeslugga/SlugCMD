use winapi::shared::windef::HDC;
use winapi::um::wingdi::{Rectangle};

pub fn draw_to_screen(hdc: HDC) {
    unsafe {
        Rectangle(hdc, 0, 0, 100, 100);
    }
}