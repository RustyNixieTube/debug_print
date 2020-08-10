#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] print!($($arg)*));
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!($($arg)*));
}
#[macro_export]
macro_rules! debug_eprint {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] eprint!($($arg)*));
}

#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] eprintln!($($arg)*));
}