use hyper::{ Response, Body };
use crate::http_server::IntoResponse;

/// An error type that allows distinguishing between client and server errors
#[derive(Debug)]
pub enum Error {
    /// Out Fault
    Us(anyhow::Error),
    /// Their fault
    Them(u16, String),
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Us(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Us(e.into())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::Us(e.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        match self {
            Error::Us(e) => e.into_response(),
            Error::Them(code, message) => {
                Response::builder()
                    .status(code)
                    .body(Body::from(message))
                    .unwrap()
            },
        }
    }
}