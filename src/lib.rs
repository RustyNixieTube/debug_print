/*!
## This crate provide prints macros that are not compiled in releases builds.

### Basic Examples
```
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
```
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
*/

/// Prints to the standard ouput only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.print.html](https://doc.rust-lang.org/std/macro.print.html) for more info.
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] print!($($arg)*));
}

/// Prints to the standard ouput only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.println.html](https://doc.rust-lang.org/std/macro.println.html) for more info.
#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}

/// Prints to the standard error only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.eprint.html](https://doc.rust-lang.org/std/macro.eprint.html) for more info.
#[macro_export]
macro_rules! debug_eprint {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] eprint!($($arg)*));
}

/// Prints to the standard error only in debug build.  
/// In release build this macro is not compiled thanks to `#[cfg(debug_assertions)]`.  
/// see [https://doc.rust-lang.org/std/macro.eprintln.html](https://doc.rust-lang.org/std/macro.eprintln.html) for more info.
#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] eprintln!($($arg)*));
}
