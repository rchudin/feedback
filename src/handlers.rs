use std::convert::Infallible;
use warp::http::StatusCode;

pub(crate) async fn status() -> Result<impl warp::Reply, Infallible> {
    Ok(StatusCode::OK)
}
