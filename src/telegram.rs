use crate::error::Error;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate)]
pub(crate) struct Message<'a> {
    #[validate(length(min = 1, max = 50))]
    pub(crate) username: &'a str,
    #[validate(length(min = 1, max = 128))]
    pub(crate) subject: &'a str,
    // #[validate(email)]
    #[validate(length(max = 512))]
    pub(crate) email: Option<&'a str>,
    // #[validate(phone)]
    #[validate(length(max = 25))]
    pub(crate) phone: Option<&'a str>,
    #[validate(length(max = 3000))]
    pub(crate) text: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response<T: Serialize> {
    ok: bool,
    result: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageInfo {
    message_id: i32,
}

pub(crate) async fn status(token: &str) -> Result<(), Error> {
    #[derive(Debug, Serialize, Deserialize)]
    struct Response {
        ok: bool,
    }

    let resp: Response = reqwest::get(&*format!(
        "https://api.telegram.org/bot{}/getUpdates",
        token
    ))
    .await?
    .json()
    .await?;

    match resp.ok {
        false => Err(Error::TelegramError),
        true => Ok(()),
    }
}

pub(crate) async fn send_message<'a>(
    token: &'a str,
    chat_id: &'a str,
    message: Message<'a>,
) -> Result<i32, Error> {
    message.validate()?;

    let mut text = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&parse_mode=html&text=<b>Subject:</b> {}%0A<b>Username:</b> {}",
        token,
        chat_id,
        message.subject,
        message.username,
    );

    if let Some(x) = message.email {
        text.push_str("%0A<b>Email:</b> ");
        text.push_str(x);
    }

    if let Some(x) = message.phone {
        text.push_str("%0A<b>Phone:</b> ");
        text.push_str(x);
    }

    if let Some(x) = message.text {
        text.push_str("%0A");
        text.push_str(x);
    }

    let resp: Response<MessageInfo> = reqwest::get(&*text).await?.json().await?;

    match resp.ok {
        false => Err(Error::TelegramError),
        true => Ok(resp.result.message_id),
    }
}

pub(crate) async fn send_document<'a>(
    token: &'a str,
    chat_id: &'a str,
    reply_to_message_id: i32,
    part: multipart::Part,
) -> Result<i32, Error> {
    let form = multipart::Form::new().part("document", part);

    let client = reqwest::Client::new();

    let resp: Response<MessageInfo> = client
        .post(&*format!(
            "https://api.telegram.org/bot{}/sendDocument?chat_id={}&reply_to_message_id={}",
            token, chat_id, reply_to_message_id
        ))
        .multipart(form)
        .send()
        .await?
        .json()
        .await?;

    match resp.ok {
        false => Err(Error::TelegramError),
        true => Ok(resp.result.message_id),
    }
}
