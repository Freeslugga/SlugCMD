use std::ffi::c_int;
use std::ptr::{null_mut};
use winapi::um::winuser;
use winapi::um::wingdi::{GetStockObject};
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, LOWORD, HIWORD};
use winapi::shared::windef::{HWND};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winuser::{CreateWindowExW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, LoadCursorW, LoadIconW, MessageBoxW, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE, RegisterClassW, MSG, TranslateMessage, DispatchMessageW, DefWindowProcW, PAINTSTRUCT, EndPaint, BeginPaint};
use crate::draw;
use crate::logger::Logger;

pub struct Window {
    pub hwnd: HWND,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        unsafe {
            let h_instance = winapi::um::libloaderapi::GetModuleHandleW(null_mut());

            let class_name: Vec<u16> = OsStr::new("MyWindowClass")
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect();

            let wnd_class = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::window_proc),
                hInstance: h_instance,
                lpszClassName: class_name.as_ptr(),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: LoadIconW(null_mut(), winapi::um::winuser::IDI_APPLICATION),
                hCursor: LoadCursorW(null_mut(), winapi::um::winuser::IDC_ARROW),
                hbrBackground: GetStockObject(winapi::um::wingdi::WHITE_BRUSH as c_int) as *mut winapi::shared::windef::HBRUSH__,
                lpszMenuName: null_mut(),
            };

            RegisterClassW(&wnd_class);

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                OsStr::new(title).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                width,
                height,
                null_mut(),
                null_mut(),
                h_instance,
                null_mut()
            );

            if hwnd.is_null() {
                let error_msg = "Error creating window";
                MessageBoxW(null_mut(), OsStr::new(error_msg).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(), null_mut(), 0);
                return Err(error_msg.to_string());
            }

            let mut window = Window { hwnd, width, height, x: 0, y: 0 };

            winuser::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, &mut window as *mut _ as isize);

            Ok(window)
        }
    }

    pub fn run_message_loop(&self) {
        unsafe {
            let mut msg: MSG = std::mem::zeroed();
            while winuser::GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

        let window_ptr = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA) as *mut Window;
        if window_ptr.is_null() {
            return DefWindowProcW(hwnd, msg, w_param, l_param);
        }
        let window = &mut *window_ptr;

        let mut logger = Logger::new();

        match msg {
            winuser::WM_DESTROY => {
                winuser::PostQuitMessage(0);
                logger.log_warning("Window has been destroyed.");
                0
            },
            winuser::WM_PAINT => {
                let mut ps: PAINTSTRUCT = std::mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);

                draw::draw_to_screen(hdc);

                EndPaint(hwnd, &ps);
                0
            },
            winuser::WM_SIZING => {
                logger.log("Window is being resized.");
                0
            },
            winuser::WM_SIZE => {
                logger.log("Window has been resized.");
                let width: isize = LOWORD(l_param as u32) as isize;
                let height: isize = HIWORD(l_param as u32) as isize;
                window.width = width as i32;
                window.height = height as i32;
                logger.log(&format!("Width: {}, Height: {}", window.width, window.height));
                0
            },
            winuser::WM_MOVE => {
                logger.log("Window is being moved.");
                let x = LOWORD(l_param as u32) as isize;
                let y = HIWORD(l_param as u32) as isize;
                window.x = x as i32;
                window.y = y as i32;
                logger.log(&format!("X: {}, Y: {}", window.x, window.y));
                0
            },
            _=> DefWindowProcW(hwnd, msg, w_param, l_param),
        }
    }
}