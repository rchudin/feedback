use validator::ValidationErrors;

#[derive(Debug)]
pub enum Error {
    ValidationErrors(ValidationErrors),
    ReqwestError(reqwest::Error),
    StaticMessage(&'static str),
    WarpError(warp::Error),
    TelegramError,
}

impl std::error::Error for Error {}

impl warp::reject::Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ValidationErrors(ref e) => e.fmt(f),
            Error::StaticMessage(ref e) => e.fmt(f),
            Error::ReqwestError(ref e) => e.fmt(f),
            Error::WarpError(ref e) => e.fmt(f),
            Error::TelegramError => write!(f, "Telegram error"),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(src: ValidationErrors) -> Self {
        Error::ValidationErrors(src)
    }
}

impl From<reqwest::Error> for Error {
    fn from(src: reqwest::Error) -> Self {
        Error::ReqwestError(src)
    }
}

impl From<warp::Error> for Error {
    fn from(src: warp::Error) -> Self {
        Error::WarpError(src)
    }
}

impl From<Error> for warp::Rejection {
    fn from(src: Error) -> Self {
        warp::reject::custom(src)
    }
}
