#[macro_use] extern crate colorify;
use std::io::{self, Write};

fn main() {
	// List colors:
	printc!(help);

	// Use one of three ways:
	printc!(yellow: "Number of banana peels on head: {} ", 7);
	printlnc!(red: "Number of zombies killed: {} ", 50);
	writeln!(io::stdout(), colorify!(orange: "Number of baggies filled \
		while walking dogs: {} "), 2).unwrap();
}