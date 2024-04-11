mod terminal;
mod popup;
mod window;
mod draw;
mod logger;

use enable_ansi_support::enable_ansi_support;
use crate::logger::Logger;
use crate::popup::Popup;
use crate::terminal::{get_terminal_size};
use crate::window::Window;
fn main() {
    // #![windows_subsystem = "windows"] TODO: Uncomment this line to hide the console window
    enable_ansi_support().unwrap();
    let mut logger = Logger::new();
    logger.log_error("This is an error message.");
    logger.log("Ansi support enabled in terminal.");
    let size = get_terminal_size().unwrap();
    logger.log(&format!("Terminal size: {}x{}", size.0, size.1));
    let popup = Popup::new(
        "braaaap".encode_utf16().chain(Some(0)).collect(),
        "Fart".encode_utf16().chain(Some(0)).collect());
    popup.open();
    let window = Window::new("I lick windows V1.0", 800, 600).unwrap();
    window.run_message_loop();
}