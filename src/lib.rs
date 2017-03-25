//! Format text printed to the terminal using ANSI colors.
//!
//!
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```text
//! [dependencies]
//! colorify = "0.2"
//! ```
//!
//! ## Example
//!
//! ```
//! #[macro_use] extern crate colorify;
//! use std::io::{self, Write};
//!
//! fn main() {
//!     // List colors:
//!     printc!(help);
//!
//!     // Use one of three ways:
//!     printc!(yellow: "Number of banana peels on head: {}", 7);
//!     printlnc!(red: "Number of zombies killed: {}", 50);
//!     writeln!(io::stdout(), colorify!(orange: "Number of baggies filled \
//!         while walking dogs: {}"), 2).unwrap();
//! }
//! ```
//!

/// Adds color to a formatting literal.
///
/// #### Usage
///
/// `writeln!(fmtr, colorify!(red: "Number of zombies killed: {}"), zombie_kills);`
///
#[cfg(not(windows))]
#[macro_export]
macro_rules! colorify {
    (help ) => ( concat!(help!(), "\n") );
    (help: ) => ( concat!(help!(), "\n") );
    (help: $s:expr) => ( concat!(help!(), "\n", "\x1b[0m", $s, "\x1b[0m") );
    (default: $s:expr) => ( concat!("\x1b[0m", $s, "\x1b[0m") );
    (black: $s:expr) => ( concat!("\x1b[30m", $s, "\x1b[0m") );
    (black_bold: $s:expr) => ( concat!("\x1b[1;30m", $s, "\x1b[0m") );
    (red: $s:expr) => ( concat!("\x1b[31m", $s, "\x1b[0m") );
    (red_bold: $s:expr) => ( concat!("\x1b[1;31m", $s, "\x1b[0m") );
    (green: $s:expr) => ( concat!("\x1b[32m", $s, "\x1b[0m") );
    (green_bold: $s:expr) => ( concat!("\x1b[1;32m", $s, "\x1b[0m") );
    (orange: $s:expr) => ( concat!("\x1b[33m", $s, "\x1b[0m") );
    (orange_bold: $s:expr) => ( concat!("\x1b[1;33m", $s, "\x1b[0m") );
    (blue: $s:expr) => ( concat!("\x1b[34m", $s, "\x1b[0m") );
    (blue_bold: $s:expr) => ( concat!("\x1b[1;34m", $s, "\x1b[0m") );
    (purple: $s:expr) => ( concat!("\x1b[35m", $s, "\x1b[0m") );
    (purple_bold: $s:expr) => ( concat!("\x1b[1;35m", $s, "\x1b[0m") );
    (cyan: $s:expr) => ( concat!("\x1b[36m", $s, "\x1b[0m") );
    (cyan_bold: $s:expr) => ( concat!("\x1b[1;36m", $s, "\x1b[0m") );
    (light_grey: $s:expr) => ( concat!("\x1b[37m", $s, "\x1b[0m") );
    (white_bold: $s:expr) => ( concat!("\x1b[1;37m", $s, "\x1b[0m") );
    (dark_grey: $s:expr) => ( concat!("\x1b[90m", $s, "\x1b[0m") );
    (dark_grey_bold: $s:expr) => ( concat!("\x1b[1;90m", $s, "\x1b[0m") );
    (peach: $s:expr) => ( concat!("\x1b[91m", $s, "\x1b[0m") );
    (peach_bold: $s:expr) => ( concat!("\x1b[1;91m", $s, "\x1b[0m") );
    (lime: $s:expr) => ( concat!("\x1b[92m", $s, "\x1b[0m") );
    (lime_bold: $s:expr) => ( concat!("\x1b[1;92m", $s, "\x1b[0m") );
    (yellow: $s:expr) => ( concat!("\x1b[93m", $s, "\x1b[0m") );
    (yellow_bold: $s:expr) => ( concat!("\x1b[1;93m", $s, "\x1b[0m") );
    (royal_blue: $s:expr) => ( concat!("\x1b[94m", $s, "\x1b[0m") );
    (royal_blue_bold: $s:expr) => ( concat!("\x1b[1;94m", $s, "\x1b[0m") );
    (magenta: $s:expr) => ( concat!("\x1b[95m", $s, "\x1b[0m") );
    (magenta_bold: $s:expr) => ( concat!("\x1b[1;95m", $s, "\x1b[0m") );
    (teal: $s:expr) => ( concat!("\x1b[96m", $s, "\x1b[0m") );
    (teal_bold: $s:expr) => ( concat!("\x1b[1;96m", $s, "\x1b[0m") );
    (white: $s:expr) => ( concat!("\x1b[97m", $s, "\x1b[0m") );
    (white_bold2: $s:expr) => ( concat!("\x1b[1;97m", $s, "\x1b[0m") );
}

#[cfg(windows)]
#[macro_export]
macro_rules! colorify {
    (help ) => ( concat!(help!(), "\n") );
    (help: ) => ( concat!(help!(), "\n") );
    (help: $s:expr) => ( concat!(help!(), "\n", "\x1b[0m", $s, "\x1b[0m") );
    (default: $s:expr) => ( $s );
    (black: $s:expr) => ( $s );
    (black_bold: $s:expr) => ( $s );
    (red: $s:expr) => ( $s );
    (red_bold: $s:expr) => ( $s );
    (green: $s:expr) => ( $s );
    (green_bold: $s:expr) => ( $s );
    (orange: $s:expr) => ( $s );
    (orange_bold: $s:expr) => ( $s );
    (blue: $s:expr) => ( $s );
    (blue_bold: $s:expr) => ( $s );
    (purple: $s:expr) => ( $s );
    (purple_bold: $s:expr) => ( $s );
    (cyan: $s:expr) => ( $s );
    (cyan_bold: $s:expr) => ( $s );
    (light_grey: $s:expr) => ( $s );
    (white_bold: $s:expr) => ( $s );
    (dark_grey: $s:expr) => ( $s );
    (dark_grey_bold: $s:expr) => ( $s );
    (peach: $s:expr) => ( $s );
    (peach_bold: $s:expr) => ( $s );
    (lime: $s:expr) => ( $s );
    (lime_bold: $s:expr) => ( $s );
    (yellow: $s:expr) => ( $s );
    (yellow_bold: $s:expr) => ( $s );
    (royal_blue: $s:expr) => ( $s );
    (royal_blue_bold: $s:expr) => ( $s );
    (magenta: $s:expr) => ( $s );
    (magenta_bold: $s:expr) => ( $s );
    (teal: $s:expr) => ( $s );
    (teal_bold: $s:expr) => ( $s );
    (white: $s:expr) => ( $s );
    (white_bold2: $s:expr) => ( $s );
}

/// `print!` with all the glorious colors of the the ANSI rainbow.
///
/// Watch out for the leprechaun at the end of that rainbow. Seriously.
///
/// #### Usage
///
/// `printc!(yellow: "Number of banana peels on head: {}", hat_height);`
///
/// See [`colorify!` docs](/colorify/colorify/macro.colorify!.html)
/// for a current list of colors.
#[macro_export]
macro_rules! printc {
    ($c:ident ) => ( print!(colorify!($c)) );
    ($c:ident: ) => ( print!(colorify!($c)) );
    ($c:ident: $fmt:expr) => ( print!(colorify!($c: $fmt)) );
    ($c:ident: $fmt:expr, $($arg:tt)*) => ( print!(colorify!($c: $fmt), $($arg)*) );
}

/// `println!` with color.
///
/// #### Usage
///
/// `printlnc!(orange: "Number of baggies filled while walking dogs: {}", bag_count);`
///
/// See [`colorify!` docs](/colorify/colorify/macro.colorify!.html)
/// for a current list of colors.
#[macro_export]
macro_rules! printlnc {
    ($c:ident ) => ( print!(colorify!($c)) );
    ($c:ident: ) => ( print!(colorify!($c)) );
    ($c:ident: $fmt:expr) => ( print!(concat!(colorify!($c: $fmt), "\n")) );
    ($c:ident: $fmt:expr, $($arg:tt)*) => ( print!(concat!(colorify!($c: $fmt), "\n"), $($arg)*) );
}

/// Returns a help string which lists all of the colors.
#[macro_export]
macro_rules! help {
     () => ( concat!(
        "[colorify! | printc! | printlnc!] Color List: {{ ",
        "\x1b[0m", "default, ", "\x1b[0m",
        "\x1b[30m", "black, ", "\x1b[0m",
        "\x1b[1;30m", "black_bold, ", "\x1b[0m",
        "\x1b[31m", "red, ", "\x1b[0m",
        "\x1b[1;31m", "red_bold, ", "\x1b[0m",
        "\x1b[32m", "green, ", "\x1b[0m",
        "\x1b[1;32m", "green_bold, ", "\x1b[0m",
        "\x1b[33m", "orange, ", "\x1b[0m",
        "\x1b[1;33m", "orange_bold, ", "\x1b[0m",
        "\x1b[34m", "blue, ", "\x1b[0m",
        "\x1b[1;34m", "blue_bold, ", "\x1b[0m",
        "\x1b[35m", "purple, ", "\x1b[0m",
        "\x1b[1;35m", "purple_bold, ", "\x1b[0m",
        "\x1b[36m", "cyan, ", "\x1b[0m",
        "\x1b[1;36m", "cyan_bold, ", "\x1b[0m",
        "\x1b[37m", "light_grey, ", "\x1b[0m",
        "\x1b[1;37m", "light_grey_bold, ", "\x1b[0m",
        "\x1b[90m", "dark_grey, ", "\x1b[0m",
        "\x1b[1;90m", "dark_grey_bold, ", "\x1b[0m",
        "\x1b[91m", "peach, ", "\x1b[0m",
        "\x1b[1;91m", "peach_bold, ", "\x1b[0m",
        "\x1b[92m", "lime, ", "\x1b[0m",
        "\x1b[1;92m", "lime_bold, ", "\x1b[0m",
        "\x1b[93m", "yellow, ", "\x1b[0m",
        "\x1b[1;93m", "yellow_bold, ", "\x1b[0m",
        "\x1b[94m", "royal_blue, ", "\x1b[0m",
        "\x1b[1;94m", "royal_blue_bold, ", "\x1b[0m",
        "\x1b[95m", "magenta, ", "\x1b[0m",
        "\x1b[1;95m", "magenta_bold, ", "\x1b[0m",
        "\x1b[96m", "teal, ", "\x1b[0m",
        "\x1b[1;96m", "teal_bold, ", "\x1b[0m",
        "\x1b[97m", "white, ", "\x1b[0m",
        "\x1b[1;97m", "white_bold. ", "\x1b[0m",
        " }} ",
        "Example: {{ ",
        "\x1b[97m", "printlnc!(peach: ",
        "\x1b[91m", "\"Ain't things just peachy with colorify!\"",
        "\x1b[97m", ")", "\x1b[0m", " }}",
    ) )
}
