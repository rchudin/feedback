use crate::error::Error;
use std::str::from_utf8;

macro_rules! vec_bytes_str_or_empty {
    ($a:expr) => {
        match $a {
            Some(ref x) => from_utf8(&x).unwrap_or(""),
            None => "",
        }
    };
}

#[derive(Default)]
pub(crate) struct Message {
    pub(crate) username: Option<Vec<u8>>,
    pub(crate) message: Option<Vec<u8>>,
    pub(crate) subject: Option<Vec<u8>>,
    pub(crate) email: Option<Vec<u8>>,
    pub(crate) phone: Option<Vec<u8>>,
}

pub(crate) async fn send_message(message: Message) -> Result<(), Error> {
    let subject = vec_bytes_str_or_empty!(message.subject);
    let username = vec_bytes_str_or_empty!(message.username);
    if subject.len() < 3 || username.len() < 3 {
        return Err(Error::ValidationErrors);
    }

    println!(
        "*Subject:* _{}_\n*Username:* _{}_\n*Email:* _{}_\n*Phone:* _{}_\n*Message:* _{}_\n",
        subject,
        username,
        vec_bytes_str_or_empty!(message.email),
        vec_bytes_str_or_empty!(message.phone),
        vec_bytes_str_or_empty!(message.message),
    );

    Ok(())
}
