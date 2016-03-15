//! Format text printed to the terminal using ANSI colors.

/// Returns a help string which lists all of the colors.
#[macro_export]
macro_rules! help {
     () => ( concat!(
        "'colorify!' / 'printc!' / 'printlnc!'  Color List: {{ ",
        "\x1b[0m", "default, ", "\x1b[0m",
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
        " }}\n"
    ) )
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
    // (help ) => ( help!() );
    // (help: ) => ( help!() );
    // (help: ($s:expr)*) => ( help!() );
    ($c:ident ) => ( print!(concat!(colorify!($c), "\n")) );
    ($c:ident: ) => ( print!(concat!(colorify!($c), "\n")) );
    ($c:ident: $fmt:expr) => ( print!(concat!(colorify!($c: $fmt), "\n")) );
    ($c:ident: $fmt:expr, $($arg:tt)*) => ( print!(concat!(colorify!($c: $fmt), "\n"), $($arg)*) );
}

/// Adds color to a formatting literal.
///
/// #### Usage
/// 
/// `writeln!(fmtr, colorify!(red: "Number of zombies killed: {}"), zombie_kills);`
///
#[macro_export]
macro_rules! colorify {
    (help ) => ( help!() );
    (help: ) => ( help!() );
    // (help: $fmt:expr) => ( help!() );
    // (help: $fmt:expr, $($arg:tt)*) => ( help!() );
    (default: $s:expr) => ( concat!("\x1b[0m", $s, "\x1b[0m") );
    (red: $s:expr) => ( concat!("\x1b[31m", $s, "\x1b[0m") );
    (red_bold: $s:expr) => ( concat!("\x1b[1;31m", $s, "\x1b[0m") );
    (green: $s:expr) => ( concat!("\x1b[32m", $s, "\x1b[0m") );
    (green_bold: $s:expr) => ( concat!("\x1b[1;32m", $s, "\x1b[0m") );
    (orange: $s:expr) => ( concat!("\x1b[33m", $s, "\x1b[0m") );
    (yellow_bold: $s:expr) => ( concat!("\x1b[1;33m", $s, "\x1b[0m") );
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
    (yellow_bold2: $s:expr) => ( concat!("\x1b[1;93m", $s, "\x1b[0m") );
    (royal_blue: $s:expr) => ( concat!("\x1b[94m", $s, "\x1b[0m") );
    (royal_blue_bold: $s:expr) => ( concat!("\x1b[1;94m", $s, "\x1b[0m") );
    (magenta: $s:expr) => ( concat!("\x1b[95m", $s, "\x1b[0m") );
    (magenta_bold: $s:expr) => ( concat!("\x1b[1;95m", $s, "\x1b[0m") );
    (teal: $s:expr) => ( concat!("\x1b[96m", $s, "\x1b[0m") );
    (teal_bold: $s:expr) => ( concat!("\x1b[1;96m", $s, "\x1b[0m") );
    (white: $s:expr) => ( concat!("\x1b[97m", $s, "\x1b[0m") );
    (white_bold2: $s:expr) => ( concat!("\x1b[1;97m", $s, "\x1b[0m") );
}
