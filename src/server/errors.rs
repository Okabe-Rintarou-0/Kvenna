use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ServerError>;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("io error")]
    IoError(#[from] io::Error),
    #[error("route url `{0}` is not configured")]
    RouteError(String),
    #[error("invalid http request")]
    PareRequestError,
}
