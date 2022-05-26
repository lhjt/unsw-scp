/// Macro to quickly construct an internal server error with an error code.
macro_rules! ise {
    ($code:expr) => {
        |e| {
            tracing::error!("exception occurred: {}", e);
            actix_web::error::ErrorInternalServerError(concat!("Internal server error: EC.", $code))
        }
    };
}

pub(crate) use ise;
