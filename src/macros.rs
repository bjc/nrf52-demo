#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        crate::log::write_fmt(format_args!($($arg)*), true);
    };
    (_) => {};
}

#[macro_export]
macro_rules! logstr {
    ($($arg:tt)*) => {
        crate::log::write_fmt(format_args!($($arg)*), false);
    };
    (_) => {};
}
