// SPDX-License-Identifier: MPL-2.0

//! Logging support.

use log::{Level, LevelFilter, Metadata, Record};
use core::str::FromStr;


const LOGGER: Logger = Logger {};

/// The log level.
///
/// FIXME: The logs should be able to be read from files in the userspace,
/// and the log level should be configurable.
pub const INIT_LOG_LEVEL: Level = Level::Warn;

macro_rules! with_color {
    ($color_code:expr, $($arg:tt)*) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[m", $color_code as u8, format_args!($($arg)*))
    }};
}

#[repr(u8)]
#[allow(dead_code)]
enum ColorCode {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

struct Logger {}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let line = record.line().unwrap_or(0);
            let path = record.target();
            let args_color = match level {
                Level::Error => ColorCode::Red,
                Level::Warn => ColorCode::Yellow,
                Level::Info => ColorCode::Green,
                Level::Debug => ColorCode::Cyan,
                Level::Trace => ColorCode::BrightBlack,
            };

            crate::console::print(with_color!(
                ColorCode::White,
                "[{path}:{line}] {args} \n",
                path = path,
                line = line,
                args = with_color!(args_color, "{}", record.args())
            ));
        }
    }

    fn flush(&self) {}
}

pub(crate) fn init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(INIT_LOG_LEVEL.to_level_filter()))
        .unwrap();
}

pub fn set_max_level(level: &str) {
    let lf = LevelFilter::from_str(level)
        .ok()
        .unwrap_or(LevelFilter::Off);
    log::set_max_level(lf);
}
