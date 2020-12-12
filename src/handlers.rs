use crate::{telegram, utility::stream_data};
use futures::stream::TryStreamExt;
use std::convert::Infallible;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Rejection, Reply,
};

pub(crate) async fn status() -> Result<impl Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn feedback(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let mut message = telegram::Message::default();

    for x in parts {
        match x.name() {
            "username" => message.username = stream_data(x).await.ok(),
            "message" => message.message = stream_data(x).await.ok(),
            "subject" => message.subject = stream_data(x).await.ok(),
            "email" => message.email = stream_data(x).await.ok(),
            "phone" => message.phone = stream_data(x).await.ok(),
            _ => {}
        };
    }

    telegram::send_message(message);

    Ok(StatusCode::OK)
}
