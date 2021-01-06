use actix_web::http::StatusCode;
use actix_web::ResponseError;
use anyhow::Error as AnyError;
use std::fmt::{self, Display, Formatter};

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

pub type Result<T, E = Error> = std::result::Result<T, E>;
