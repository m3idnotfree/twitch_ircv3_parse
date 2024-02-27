use nom::{
    bytes::complete::{tag, take_until},
    sequence::preceded,
    IResult,
};
use serde::Serialize;

use crate::{emotes, BadgeData};

#[derive(Debug, Serialize)]
pub struct ChatInfo {
    #[serde(rename = "type")]
    pub kind: String,
    pub time: Option<String>,
    pub badges: Option<Vec<BadgeData>>,
    pub message: Option<Vec<emotes::Message>>,
}

impl ChatInfo {
    pub fn new<T: Into<String>>(
        kind: T,
        time: Option<String>,
        badges: Option<Vec<BadgeData>>,
        message: Option<Vec<emotes::Message>>,
    ) -> ChatInfo {
        ChatInfo {
            kind: kind.into(),
            time,
            badges,
            message,
        }
    }
    pub fn no_tag_new() {}
}
pub fn is_capabilities(msg: &str) -> IResult<&str, bool> {
    let (_, cap) = preceded(tag(" * "), take_until(" "))(msg)?;
    let d;
    if cap == "ACK" {
        d = true
    } else {
        d = false;
    }

    Ok(("", d))
}
