use crate::error::Error;
use validator::Validate;

#[derive(Default, Validate)]
pub(crate) struct Message<'a> {
    #[validate(length(min = 3, max = 256))]
    pub(crate) username: &'a str,
    #[validate(length(min = 3, max = 512))]
    pub(crate) subject: &'a str,
    #[validate(email)]
    pub(crate) email: Option<&'a str>,
    #[validate(phone)]
    pub(crate) phone: Option<&'a str>,
    #[validate(length(min = 0, max = 3000))]
    pub(crate) text: Option<&'a str>,
}

pub(crate) async fn send_message<'a>(
    token: &'a str,
    chat_id: &'a str,
    message: Message<'a>,
) -> Result<(), Error> {
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

    reqwest::get(&*text).await?;

    Ok(())
}

pub(crate) async fn status(token: &str) -> Result<(), ()> {
    let resp = match reqwest::get(&*format!(
        "https://api.telegram.org/bot{}/getUpdates",
        token
    ))
    .await
    {
        Ok(x) => x,
        Err(_) => return Err(()),
    };
    if resp.status() != 200 {
        return Err(());
    }

    Ok(())
}
