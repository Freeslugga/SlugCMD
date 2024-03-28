use std::fmt;
use std::fmt::format;

pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    LGreen,
    LBlue,
    LRed,
    LYellow,
    LCyan,
    LMagenta,
}

pub fn convert_fg_color_to_string(color: Color) -> String {
    match color {
        Color::White => "37".to_string(),
        Color::Black => "30".to_string(),
        Color::Red => "31".to_string(),
        Color::Green => "32".to_string(),
        Color::Yellow => "33".to_string(),
        Color::Blue => "34".to_string(),
        Color::Magenta => "35".to_string(),
        Color::Cyan => "36".to_string(),
        Color::LRed => "91".to_string(),
        Color::LGreen => "92".to_string(),
        Color::LBlue => "94".to_string(),
        Color::LYellow => "93".to_string(),
        Color::LCyan => "96".to_string(),
        Color::LMagenta => "95".to_string(),
    }
}

pub fn get_current_bg_color() -> Color {
    Color::Red
}

pub fn create_color_string(msg: String, color: Color) -> String {
    let col_string = convert_fg_color_to_string(color);

    let formatted_msg = format!("\x1b[{col_string}m{msg}\x1b[0m").to_string();

    formatted_msg
}