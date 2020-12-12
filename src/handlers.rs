use crate::{telegram, utility::stream_data};
use futures::stream::TryStreamExt;
use std::convert::Infallible;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Rejection, Reply,
};

macro_rules! bytes_as_str_or_empty {
    ($a:expr) => {
        match $a {
            Some(ref x) => std::str::from_utf8(&x).unwrap_or(""),
            None => "",
        }
    };
}

macro_rules! bytes_as_str_or_none {
    ($a:expr) => {
        match $a {
            Some(ref x) => std::str::from_utf8(&x).ok(),
            None => None,
        }
    };
}

pub(crate) async fn status() -> Result<impl Reply, Infallible> {
    Ok(StatusCode::OK)
}

pub(crate) async fn feedback(form: FormData) -> Result<impl Reply, Rejection> {
    #[derive(Default)]
    pub(crate) struct Form {
        pub(crate) username: Option<Vec<u8>>,
        pub(crate) subject: Option<Vec<u8>>,
        pub(crate) email: Option<Vec<u8>>,
        pub(crate) phone: Option<Vec<u8>>,
        pub(crate) text: Option<Vec<u8>>,
    };

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let mut message = Form::default();

    for x in parts {
        match x.name() {
            "username" => message.username = stream_data(x).await.ok(),
            "subject" => message.subject = stream_data(x).await.ok(),
            "email" => message.email = stream_data(x).await.ok(),
            "phone" => message.phone = stream_data(x).await.ok(),
            "message" => message.text = stream_data(x).await.ok(),
            _ => {}
        };
    }

    telegram::send_message(
        "",
        "",
        telegram::Message {
            username: bytes_as_str_or_empty!(message.username),
            subject: bytes_as_str_or_empty!(message.subject),
            email: bytes_as_str_or_none!(message.email),
            phone: bytes_as_str_or_none!(message.phone),
            text: bytes_as_str_or_none!(message.text),
        },
    )
    .await?;

    Ok(StatusCode::OK)
}
