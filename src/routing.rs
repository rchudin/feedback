use crate::{error::Error, handlers, state::State};
use std::{convert::Infallible, sync::Arc};
use warp::{filters::body::BodyDeserializeError, http::StatusCode, Filter, Rejection, Reply};

macro_rules! path {
    () => {
        warp::path!("api" / "feedback" )
    };
    ($($a:tt)*) => {
        warp::path!("api" / "feedback" /  $($a)*)
    };
}

pub(crate) fn routing(
    state: State,
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    status().or(feedback(state)).recover(rejection)
}

pub fn status() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!().and(warp::get()).and_then(handlers::status)
}

pub fn feedback(state: State) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path!()
        .and(warp::post())
        .and(with_state(Arc::new(state)))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(handlers::feedback)
}

fn with_state(
    state: Arc<State>,
) -> impl Filter<Extract = (Arc<State>,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

async fn rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(e) = err.find::<Error>() {
        match e {
            _ => (StatusCode::BAD_REQUEST, "BAD REQUEST"),
        }
    } else if err.find::<BodyDeserializeError>().is_some() {
        (StatusCode::BAD_REQUEST, "Bad Request")
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
