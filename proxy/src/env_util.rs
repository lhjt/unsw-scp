/// Create a static variable through an environment variable.
macro_rules! lazy_env {
    ($var:expr, $default:expr) => {
        once_cell::sync::Lazy::new(|| match env::var($var) {
            Ok(v) => v,
            Err(_) => $default.to_string(),
        })
    };
}

pub(crate) use lazy_env;
