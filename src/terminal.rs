use std::io;
use std::mem;
use winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO};
use winapi::um::winnt::HANDLE;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::shared::minwindef::DWORD;

pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    unsafe {
        let stdout_handle = winapi::um::processenv::GetStdHandle(STD_OUTPUT_HANDLE);
        if stdout_handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }

        let mut console_info: CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        if GetConsoleScreenBufferInfo(stdout_handle as HANDLE, &mut console_info) == 0 {
            return Err(io::Error::last_os_error());
        }

        let width = console_info.dwSize.X as u16;
        let height = console_info.dwSize.Y as u16;
        Ok((width, height))
    }
}

pub fn set_terminal_handle(hdl: DWORD) -> bool {

}

pub struct Terminal {
    window_size: (u32, u32),
    cursor_position: (u32, u32),
    terminal_handle: DWORD
}