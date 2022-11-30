//! Terminal colors library.
//!
//! Contains functions that generate data structure with preset terminal color
//! and attribute string values to allow for easy use with standard print
//! functions. ANSI 16 colors and basic style attributes only. By default all
//! values are set to empty string if `NO_COLOR` environment variable is set or
//! if program is not running inside of interactive TTY, i.e. colors are
//! automatically disabled during redirection or piping.
//!
//! Use function [`init_auto`] for recommended default behaviour. Functions
//! [`init_on`] and [`init_off`] can be used to enforce specific behaviour,
//! e.g. to support implementation of `--color=on/off` argument.
//!
//! Structure:
//!
//! ```text
//! colors
//! |-- attr
//! |   |-- blink
//! |   |-- bold
//! |   |-- italic
//! |   |-- reset
//! |   |-- reverse
//! |   `-- underline
//! |-- bg
//! |   |-- black
//! |   |-- blue
//! |   |-- cyan
//! |   |-- green
//! |   |-- magenta
//! |   |-- red
//! |   |-- white
//! |   |-- yellow
//! |   |-- bright_black
//! |   |-- bright_blue
//! |   |-- bright_cyan
//! |   |-- bright_green
//! |   |-- bright_magenta
//! |   |-- bright_red
//! |   |-- bright_white
//! |   `-- bright_yellow
//! `-- fg
//!     |-- black
//!     |-- blue
//!     |-- cyan
//!     |-- green
//!     |-- magenta
//!     |-- red
//!     |-- white
//!     |-- yellow
//!     |-- bright_black
//!     |-- bright_blue
//!     |-- bright_cyan
//!     |-- bright_green
//!     |-- bright_magenta
//!     |-- bright_red
//!     |-- bright_white
//!     `-- bright_yellow
//! ```
//!
//! Usage:
//!
//! ```rust,ignore
//! let term = colors::init_auto();
//! println!("{}Hello, 世界{}", term.fg.red, term.attr.reset);
//! ```
//!
//! Author: Göran Gustafsson <gustafsson.g@gmail.com>
//!
//! License: BSD 3-Clause

use std::env;

/// Terminal style attributes.
#[derive(Default)]
pub struct Attributes {
    pub blink: String,
    pub bold: String,
    pub italic: String,
    pub reset: String,
    pub reverse: String,
    pub underline: String,
}

/// Terminal background & foreground colors.
#[derive(Default)]
pub struct Colors {
    pub black: String,
    pub blue: String,
    pub cyan: String,
    pub green: String,
    pub magenta: String,
    pub red: String,
    pub white: String,
    pub yellow: String,

    pub bright_black: String,
    pub bright_blue: String,
    pub bright_cyan: String,
    pub bright_green: String,
    pub bright_magenta: String,
    pub bright_red: String,
    pub bright_white: String,
    pub bright_yellow: String,
}

/// Data structure containing all attributes and colors.
#[derive(Default)]
pub struct Codes {
    pub attr: Attributes,
    pub bg: Colors,
    pub fg: Colors,
}

/// Check if running inside of TTY using libc isatty().
fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

/// Check if `NO_COLOR` environment variable is set.
fn no_color_env() -> bool {
    env::var("NO_COLOR").is_ok()
}

/// Run [`init_on`] or [`init_off`] and return result from function.
///
/// If program is running inside of interactive TTY and `NO_COLOR` environment
/// variable is not set use function [`init_on`], otherwise use [`init_off`].
pub fn init_auto() -> Codes {
    if is_tty() && !no_color_env() {
        return init_on();
    }

    init_off()
}

/// Return data structure with preset attribute and color values.
pub fn init_on() -> Codes {
    Codes {
        #[rustfmt::skip]
        attr: Attributes {
            reset:     "\x1B[0m".to_string(),
            bold:      "\x1B[1m".to_string(),
            italic:    "\x1B[3m".to_string(),
            underline: "\x1B[4m".to_string(),
            blink:     "\x1B[5m".to_string(),
            reverse:   "\x1B[7m".to_string(),
        },
        #[rustfmt::skip]
        bg: Colors {
            black:   "\x1B[40m".to_string(),
            red:     "\x1B[41m".to_string(),
            green:   "\x1B[42m".to_string(),
            yellow:  "\x1B[43m".to_string(),
            blue:    "\x1B[44m".to_string(),
            magenta: "\x1B[45m".to_string(),
            cyan:    "\x1B[46m".to_string(),
            white:   "\x1B[47m".to_string(),

            bright_black:   "\x1B[100m".to_string(),
            bright_red:     "\x1B[101m".to_string(),
            bright_green:   "\x1B[102m".to_string(),
            bright_yellow:  "\x1B[103m".to_string(),
            bright_blue:    "\x1B[104m".to_string(),
            bright_magenta: "\x1B[105m".to_string(),
            bright_cyan:    "\x1B[106m".to_string(),
            bright_white:   "\x1B[107m".to_string(),
        },
        #[rustfmt::skip]
        fg: Colors {
            black:   "\x1B[30m".to_string(),
            red:     "\x1B[31m".to_string(),
            green:   "\x1B[32m".to_string(),
            yellow:  "\x1B[33m".to_string(),
            blue:    "\x1B[34m".to_string(),
            magenta: "\x1B[35m".to_string(),
            cyan:    "\x1B[36m".to_string(),
            white:   "\x1B[37m".to_string(),

            bright_black:   "\x1B[90m".to_string(),
            bright_red:     "\x1B[91m".to_string(),
            bright_green:   "\x1B[92m".to_string(),
            bright_yellow:  "\x1B[93m".to_string(),
            bright_blue:    "\x1B[94m".to_string(),
            bright_magenta: "\x1B[95m".to_string(),
            bright_cyan:    "\x1B[96m".to_string(),
            bright_white:   "\x1B[97m".to_string(),
        },
    }
}

/// Return data structure with empty attribute and color values.
pub fn init_off() -> Codes {
    // Use default type values, i.e. empty strings.
    Codes::default()
}
