use crate::{
    state::State,
    telegram,
    utility::{reqwest_file_part, stream_data},
};
use futures::stream::TryStreamExt;
use std::{convert::Infallible, sync::Arc};
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Rejection, Reply,
};

const OK: &str = "OK";
const FILE_NOT_SENT: &str = "file not sent";

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

pub(crate) async fn feedback(state: Arc<State>, form: FormData) -> Result<impl Reply, Rejection> {
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
    let mut part_file: Option<Part> = None;

    for x in parts {
        match x.name() {
            "username" => message.username = stream_data(x).await.ok(),
            "subject" => message.subject = stream_data(x).await.ok(),
            "email" => message.email = stream_data(x).await.ok(),
            "phone" => message.phone = stream_data(x).await.ok(),
            "message" => message.text = stream_data(x).await.ok(),
            "file" => part_file = Some(x),
            _ => {}
        };
    }

    let message_id = telegram::send_message(
        &*state.token,
        &*state.chat_id,
        telegram::Message {
            username: bytes_as_str_or_empty!(message.username),
            subject: bytes_as_str_or_empty!(message.subject),
            email: bytes_as_str_or_none!(message.email),
            phone: bytes_as_str_or_none!(message.phone),
            text: bytes_as_str_or_none!(message.text),
        },
    )
    .await?;

    if let Some(part) = part_file {
        if let Ok(part) = reqwest_file_part(part).await {
            if let Ok(_) =
                telegram::send_document(&*state.token, &*state.chat_id, message_id, part).await
            {
                return Ok(warp::reply::with_status(OK, StatusCode::OK));
            }
        }
        return Ok(warp::reply::with_status(
            FILE_NOT_SENT,
            StatusCode::BAD_REQUEST,
        ));
    }

    Ok(warp::reply::with_status(OK, StatusCode::OK))
}
