use crate::handlers;
use std::convert::Infallible;
use warp::{http::StatusCode, Filter, Rejection, Reply};

macro_rules! path {
    () => {
        warp::path!("api" / "feedback" )
    };
    ($($a:tt)*) => {
        warp::path!("api" / "feedback" /  $($a)*)
    };
}

pub(crate) fn routing() -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    status().or(feedback()).recover(rejection)
}

pub fn status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!().and(warp::get()).and_then(handlers::status)
}

pub fn feedback() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!()
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(handlers::feedback)
}

async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large")
    } else {
        eprintln!("unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    Ok(warp::reply::with_status(message, code))
}
