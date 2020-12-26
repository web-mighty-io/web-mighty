use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    PoolError(PoolError),
    Error(StatusCode, String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PoolError(e) => write!(f, "{}", e),
            Error::Error(c, m) => write!(f, "{}:{}", c, m),
        }
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::PoolError(e)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::from(PoolError::from(e))
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::PoolError(err) => HttpResponse::InternalServerError().body(err.to_string()),
            Error::Error(code, msg) => HttpResponseBuilder::new(*code).body(msg.clone()),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new<S: AsRef<str>>(code: StatusCode, msg: S) -> Error {
        Error::Error(code, msg.as_ref().to_owned())
    }

    pub fn result<T, S: AsRef<str>>(code: StatusCode, msg: S) -> Result<T> {
        Err(Error::Error(code, msg.as_ref().to_owned()))
    }
}
