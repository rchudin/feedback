use crate::handlers;
use warp::Filter;

macro_rules! path {
    () => {
        warp::path!("api" / "feedback" )
    };
    ($($a:tt)*) => {
        warp::path!("api" / "feedback" /  $($a)*)
    };
}

pub(crate) fn routing() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    status()
}

pub fn status() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    path!().and(warp::get()).and_then(handlers::status)
}
