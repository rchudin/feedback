#[derive(Debug)]
pub enum Error {
    ValidationErrors,
}

impl std::error::Error for Error {}

impl warp::reject::Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ValidationErrors => write!(f, "invalid data"),
        }
    }
}

impl From<Error> for warp::Rejection {
    fn from(src: Error) -> Self {
        warp::reject::custom(src)
    }
}
