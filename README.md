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

## Windows

By default, colorify does nothing when compiled for windows targets due to
traditional incompatability. The Windows 10 console host now has the ability
to interpret ANSI escape codes and print in color. Using colorify versions
0.2.3 and above, enabling the `enable_windows` feature will cause colorify to
act exactly like it does on other operating system targets:

```
[dependencies]
colorify = { version = "~0.2.3", features = ["enable_windows"] }
```

Your windows console must already have ANSI colors enabled for this to work.
Certain powershell modules such as
[posh-git](https://github.com/dahlbyk/posh-git) will enable this for you.
There are probably ways to enable it via a registry key or within a powershell
profile script. Please submit an issue if you know of such a way and it will
be added to this page.

If there is enough interest (file an issue if you care), adding an additional
macro to enable ANSI color using
[`winapi`](https://github.com/retep998/winapi-rs) will be added to this crate
(perhaps `colorify_enable_windows!()` or something like that). Better yet, add
it and sumbit a pull request yourself!


## Semi-Retired

This library is somewhat, sort of, unlikely to be updated with new features or
changes. Maybe.

Recommended alternatives:

* [rust-ansi-term](https://github.com/ogham/rust-ansi-term)
* [term](https://github.com/Stebalien/term)
* [colored](https://github.com/mackwic/colored)
* [termcolor](https://github.com/BurntSushi/ripgrep/tree/master/termcolor)

