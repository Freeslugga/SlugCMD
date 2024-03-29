mod color;
mod terminal;
mod popup;
mod window;

use enable_ansi_support::enable_ansi_support;
use crate::color::{Color, create_color_string};
use crate::popup::Popup;
use crate::terminal::{get_terminal_size};

fn main() {
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
    while true {

    }
}