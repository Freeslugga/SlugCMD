mod color;
mod terminal;
mod popup;
mod window;

use std::ptr::{null, null_mut};
use enable_ansi_support::enable_ansi_support;
use crate::color::{Color, create_color_string};
use crate::popup::Popup;
use crate::terminal::{get_terminal_size};
use crate::window::Window;
fn main() {
    #![windows_subsystem = "windows"]
    enable_ansi_support().unwrap();
    println!("Ansi support enabled in terminal.");
    let str = create_color_string("This is a test".to_string(), Color::LMagenta);
    println!("{}", create_color_string("Hello, world!".to_string(), Color::LCyan));
    println!("{}", str);
    let size = get_terminal_size().unwrap();
    println!("X: {}, Y: {}", size.0, size.1);
    let popup = Popup::new(
        "Test".encode_utf16().chain(Some(0)).collect(),
        "Fart".encode_utf16().chain(Some(0)).collect());
    popup.open();
    let mut window = Window::new("I lick windows V1.0", 800, 600).unwrap();
    window.run_message_loop();
}