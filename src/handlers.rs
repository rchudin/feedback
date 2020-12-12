use std::convert::Infallible;
use warp::{http::StatusCode, multipart::FormData, Rejection, Reply};

pub(crate) async fn status() -> Result<impl Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn feedback(_form: FormData) -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}
