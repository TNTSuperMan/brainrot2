use std::{sync::OnceLock, time::Instant};

pub static START: OnceLock<Instant> = OnceLock::new();

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        if cfg!(feature = "debug") {
            eprintln!($($arg)*)
        }
    };
}

#[macro_export]
macro_rules! timeline {
    ($($arg:tt)*) => {
        if cfg!(feature = "debug") {
            eprintln!("[{:?}] {}", $crate::log::START.get().unwrap().elapsed(), format!($($arg)*));
        }
    };
}

pub fn start() {
    START.set(Instant::now()).unwrap();
}
