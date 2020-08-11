[![crates.io](https://img.shields.io/crates/v/debug_print.svg)](https://crates.io/crates/debug_print)
[![Documentation](https://docs.rs/debug_print/badge.svg)](https://docs.rs/debug_print)
[![License](https://img.shields.io/badge/license-MIT-0fff0f.svg)](https://opensource.org/licenses/MIT)
[![License](https://img.shields.io/badge/license-APACHE-0fff0f.svg)](https://www.apache.org/licenses/LICENSE-2.0)
![line](https://tokei.rs/b1/github/RustyNixieTube/debug_print)
# debug_print
## Examples
```rust
use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};

let x = 5 * 2;

debug_println!("x = {}", x);
debug_print!("x");
debug_print!(" = ");
debug_print!("{}", x);
debug_eprintln!("I'm printing to the Standard Error");
debug_eprint!("I'm printing to the Standard Error");
```

### If you don't like the names of the macros.
```rust
use debug_print::{
    debug_print as dprint,
    debug_println as dprintln,
    debug_eprint as deprint,
    debug_eprintln as deprintln,
};

let x = 5 * 2;

dprintln!("x = {}", x);
dprint!("x");
dprint!(" = ");
dprint!("{}", x);
deprintln!("I'm printing to the Standard Error");
deprint!("I'm printing to the Standard Error");
```
## Contribution
contributions are welcome!

## License
Licensed under either of [Apache License Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT license](https://opensource.org/licenses/MIT) at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.