use nom::{
    bytes::complete::{tag, take_until},
    sequence::preceded,
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::{emotes, BadgeData};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInfo<'a> {
    #[serde(rename = "type")]
    pub kind: &'a str,
    pub time: Option<&'a str>,
    pub badges: Option<Vec<BadgeData>>,
    pub message: Option<Vec<emotes::Message>>,
}

impl<'a> ChatInfo<'a> {
    // pub fn new<T: Into<String>>(
    pub fn new(
        kind: &'a str,
        time: Option<&'a str>,
        badges: Option<Vec<BadgeData>>,
        message: Option<Vec<emotes::Message>>,
    ) -> ChatInfo<'a> {
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
