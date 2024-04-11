use chrono::Local;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Logger {
    h_console: StandardStream,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            h_console: StandardStream::stdout(ColorChoice::Always),
        }
    }

    pub fn reset_color(&mut self) {
        self.h_console.set_color(&ColorSpec::new()).unwrap();
    }

    pub fn log(&mut self, message: &str) {
        self.h_console.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
        writeln!(&mut self.h_console, "{}: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message).unwrap();
        self.reset_color();
    }

    pub fn log_error(&mut self, message: &str) {
        self.h_console.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        writeln!(&mut self.h_console, "{}: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message).unwrap();
        self.reset_color();
    }

    pub fn log_warning(&mut self, message: &str) {
        self.h_console.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        writeln!(&mut self.h_console, "{}: {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message).unwrap();
        self.reset_color();
    }
}

impl Clone for Logger {
    fn clone(&self) -> Logger {
        Logger {
            h_console: StandardStream::stdout(ColorChoice::Always),
        }
    }
}