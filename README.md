# colorify!

#### [Documentation](http://doc.cogciprocate.com/colorify/colorify)

[![](http://meritbadge.herokuapp.com/colorify)](https://crates.io/crates/colorify)

Handy macros for printing to the terminal in color.

## Usage

Add the following to your `Cargo.toml`:

```
[dependencies]
colorify = "0.2"
```

## Example

```rust
#[macro_use] extern crate colorify;
use std::io::{self, Write};

fn main() {
	// List colors:
	printc!(help);

	// Use one of three ways:
	printc!(yellow: "Number of banana peels on head: {}", 7);
	printlnc!(red: "Number of zombies killed: {}", 50);
	writeln!(io::stdout(), colorify!(orange: "Number of baggies filled \
		while walking dogs: {}"), 2).unwrap();
}
```

## Retired

This library is unlikely to be updated with new features or changes.

Recommended alternatives:

* [term](https://github.com/Stebalien/term)
* [rust-ansi-term](https://github.com/ogham/rust-ansi-term)
* [colored](https://github.com/mackwic/colored)
* [termcolor](https://github.com/BurntSushi/ripgrep/tree/master/termcolor)

