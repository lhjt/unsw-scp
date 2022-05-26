#![allow(unused)]

/// Create a static variable through an environment variable.
#[macro_export]
macro_rules! lazy_env {
    ($var:expr, $default:expr) => {
        once_cell::sync::Lazy::new(|| match env::var($var) {
            Ok(v) => v,
            Err(_) => $default.to_string(),
        })
    };
}

/// Fetches an environment variable on startup. If it is not present, panics.
#[macro_export]
macro_rules! panic_env {
    ($var:ident) => {
        static $var: Lazy<String> =
            once_cell::sync::Lazy::new(|| match env::var(stringify!($var)) {
                Ok(v) => v,
                Err(_) => panic!(concat!(
                    "The environment variable ",
                    concat!(stringify!($var), " has not been set!")
                )),
            });
    };
}

// pub(crate) use lazy_env;
// pub(crate) use panic_env;
