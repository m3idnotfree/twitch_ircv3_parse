use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use serde::Serialize;

use crate::utils::{Emote, Message, Normal};

// mod utils;
// pub use utils::*;

#[derive(Debug, Serialize, PartialEq)]
pub struct EmotesTag(Option<Vec<(String, u16, u16)>>);

impl AsRef<Option<Vec<(String, u16, u16)>>> for EmotesTag {
    fn as_ref(&self) -> &Option<Vec<(String, u16, u16)>> {
        &self.0
    }
}

impl EmotesTag {
    pub fn new(emotes: Option<Vec<(String, u16, u16)>>) -> Self {
        Self(emotes)
    }

    pub fn output(self, msg: &str) -> Option<Vec<Message>> {
        match self.0 {
            None => None,
            Some(emotes_list) => {
                let mut result = vec![];
                let mut cur = 0;
                let mut remain = msg;

                for (emote, start_emote, end_emote) in emotes_list.into_iter() {
                    let cdn_url = EmotesTag::cdn_url(emote.as_str());

                    let start = start_emote - cur;
                    let expeced_location = end_emote - start_emote + 1;

                    let (remain2, (prev, expected)) =
                        EmotesTag::find_location(remain, start, expeced_location).unwrap();

                    remain = remain2;
                    cur = end_emote + 1;

                    if !(prev.is_empty()
                        || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
                    {
                        result.push(Message::Normal(Normal::new(prev)));
                    };

                    result.push(Message::Emote(Emote::new(
                        emote,
                        expected.to_string(),
                        cdn_url,
                    )))
                }

                if !remain.is_empty() {
                    result.push(Message::Normal(Normal::new(remain)));
                }

                Some(result)
            }
        }
    }

    pub fn find_location(msg: &str, start: u16, end: u16) -> IResult<&str, (&str, &str)> {
        tuple((take(start), take(end)))(msg)
    }

    pub fn cdn_url(emote: &str) -> String {
        format!(
            "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
            emote
        )
    }
}

pub fn parse_emotes(msg: &str) -> EmotesTag {
    let (_, result) = parse(msg).unwrap();

    EmotesTag(result)
}

fn parse(msg: &str) -> IResult<&str, Option<Vec<(String, u16, u16)>>> {
    let (_, result) = opt(separated_list1(tag("/"), emotes_set))(msg)?;

    let result = result.map(|value| {
        let mut r = value
            .into_iter()
            .flatten()
            .collect::<Vec<(String, u16, u16)>>();
        // .collect::<Vec<(&str, u16, u16)>>();
        r.sort_by(|a, b| a.1.cmp(&b.1));
        r
    });

    Ok(("", result))
}
fn emotes_set(msg: &str) -> IResult<&str, Vec<(String, u16, u16)>> {
    let (msg, (emote_id, locations)) = tuple((
        terminated(take_until(":"), tag(":")),
        separated_list0(tag(","), start_end),
    ))(msg)?;

    let result = locations
        .into_iter()
        .map(|(start, end)| (emote_id.to_string(), start, end))
        // .map(|(start, end)| (emote_id, start, end))
        .collect::<Vec<(String, u16, u16)>>();

    Ok((msg, result))
}

// start , end
fn start_end(msg: &str) -> IResult<&str, (u16, u16)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u16>()),
        tag("-"),
        map_res(digit1, |s: &str| s.parse::<u16>()),
    )(msg)
}
