use crate::error::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate)]
pub(crate) struct Message<'a> {
    #[validate(length(min = 1, max = 256))]
    pub(crate) username: &'a str,
    #[validate(length(min = 1, max = 512))]
    pub(crate) subject: &'a str,
    // #[validate(email)]
    #[validate(length(max = 512))]
    pub(crate) email: Option<&'a str>,
    // #[validate(phone)]
    #[validate(length(max = 50))]
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

pub(crate) async fn send_message<'a>(
    token: &'a str,
    chat_id: &'a str,
    message: Message<'a>,
) -> Result<i32, Error> {
    message.validate()?;

    let mut text = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&parse_mode=markdown&text=*Subject:* _{}_%0A*Username:* _{}",
        token,
        chat_id,
        message.subject,
        message.username,
    );

    if let Some(x) = message.email {
        text.push_str("_%0A*Email:* _");
        text.push_str(x);
    }

    if let Some(x) = message.phone {
        text.push_str("_%0A*Phone:* _");
        text.push_str(x);
    }

    if let Some(x) = message.text {
        text.push_str("_%0A_");
        text.push_str(x);
    }
    text.push_str("_");

    let resp: Response<MessageInfo> = reqwest::get(&*text).await?.json().await?;

    Ok(resp.result.message_id)
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
