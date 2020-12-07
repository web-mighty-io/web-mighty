use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::Display;
use std::time::Duration;

pub mod user;

const TOKEN_VALID_DURATION: Duration = Duration::from_secs(24 * 60 * 60);

#[derive(Debug, Display)]
pub enum Error {
    PoolError(PoolError),
    Error(StatusCode, String),
}

pub type Result<T> = std::result::Result<T, Error>;

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
    pub fn new<T, S: AsRef<str>>(code: StatusCode, msg: S) -> Result<T> {
        Err(Error::Error(code, msg.as_ref().to_owned()))
    }
}
