use actix_web::http::StatusCode;
use actix_web::ResponseError;
pub use anyhow as _anyhow;
use anyhow::Error as AnyError;
use std::fmt::{self, Display, Formatter};

/// Error of this server
///
/// By using `anyhow`, it can handle error easily & status code by using `StatusCode`.
#[derive(Debug)]
pub struct Error(pub StatusCode, pub AnyError);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.0
    }
}

impl<E> From<E> for Error
where
    E: Into<AnyError>,
{
    fn from(e: E) -> Self {
        Error(StatusCode::INTERNAL_SERVER_ERROR, e.into())
    }
}

/// Result type of this server
///
/// It uses `Error` by default.
pub type Result<T, E = Error> = std::result::Result<T, E>;
