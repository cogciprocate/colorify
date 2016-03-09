# colorify!

#### [Documentation](http://doc.cogciprocate.com/colorify/colorify)

[![](http://meritbadge.herokuapp.com/ocl)](https://crates.io/crates/colorify)

Handy macros for printing to the terminal in color.

## Examples

```rust
printc!(yellow: "Number of banana peels on head: {}", slick_hat_height);
printlnc!(orange: "Number of baggies filled while walking dogs: {}", bag_count);
writeln!(fmtr, colorify!(red: "Number of zombies killed: {}"), zombie_kills);
```
