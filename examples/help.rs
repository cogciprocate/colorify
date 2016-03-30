#[macro_use] extern crate colorify;
use std::io::{self, Write};

fn main() {
	printc!(help);
	printc!(help: "A string literal\n");
	printlnc!(help);
	printlnc!(help: "A string literal");
	write!(io::stdout(), colorify!(help)).unwrap();
	write!(io::stdout(), colorify!(help: "A string literal\n")).unwrap();
}
