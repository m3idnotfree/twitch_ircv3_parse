use nom::{
    bytes::complete::{tag, take_until},
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};
// use twitch_api::emotes::Emote;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct EmotesTag<'a> {
    pub data: &'a str,
}

impl<'a> EmotesTag<'a> {
    pub fn new(data: &'a str) -> EmotesTag {
        EmotesTag { data }
    }

    // emote_id, start, end, cdn
    pub fn parse(msg: &str) -> Option<Vec<(&str, u64, u64)>> {
        let (_, result) = opt(separated_list1(tag("/"), emotes_set))(msg).unwrap();
        match result {
            Some(value) => {
                let mut value = value
                    .into_iter()
                    .flatten()
                    .collect::<Vec<(&str, u64, u64)>>();

                value.sort_by(|a, b| a.1.cmp(&b.1));

                Some(value)
            }
            None => None,
        }
    }

    pub fn parse_string(msg: &str) -> Option<Vec<(String, u64, u64)>> {
        let (_, result) = opt(separated_list1(tag("/"), emotes_set_string))(msg).unwrap();
        match result {
            Some(value) => {
                let mut value = value
                    .into_iter()
                    .flatten()
                    .collect::<Vec<(String, u64, u64)>>();

                value.sort_by(|a, b| a.1.cmp(&b.1));

                Some(value)
            }
            None => None,
        }
    }

    // pub fn parse_message_with_emotes_cdn(
    //     msg: &'a str,
    //     emotes: Option<Vec<(String, u64, u64)>>,
    // ) -> IResult<&'a str, Option<Vec<MessageCDN>>> {
    //     match emotes {
    //         None => Ok(("", None)),
    //         Some(emotes_list) => {
    //             let mut result = vec![];
    //             let mut cur = 0;
    //             let mut remain = msg;
    //
    //             for (emote, start_emote, end_emote) in emotes_list.into_iter() {
    //                 let cdn_url = format!(
    //                     "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
    //                     emote
    //                 );
    //                 let start = start_emote - cur;
    //                 let expeced_location = end_emote - start_emote + 1;
    //
    //                 let (remain2, (prev, expected)) =
    //                     tuple((take(start), take(expeced_location)))(remain)?;
    //
    //                 remain = remain2;
    //                 cur = end_emote + 1;
    //
    //                 if !(prev.is_empty()
    //                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
    //                 {
    //                     result.push(MessageCDN::Normal(Normal::new(prev)));
    //                 };
    //
    //                 result.push(MessageCDN::Emote(EmoteCDN::new(
    //                     emote,
    //                     expected.to_string(),
    //                     cdn_url,
    //                 )))
    //             }
    //
    //             // if remain.len() != 0 {
    //             if !remain.is_empty() {
    //                 result.push(MessageCDN::Normal(Normal::new(remain)));
    //             }
    //             Ok(("", Some(result)))
    //         }
    //     }
    // }

    // pub fn get_data_cdn_url(
    //     msg: &'a str,
    //     emotes: Option<Vec<(&str, u64, u64)>>,
    // ) -> IResult<&'a str, Option<Vec<MessageCDN>>> {
    //     match emotes {
    //         None => Ok(("", None)),
    //         Some(emotes_list) => {
    //             let mut result = vec![];
    //             let mut cur = 0;
    //             let mut remain = msg;
    //
    //             for (emote, start_emote, end_emote) in emotes_list.into_iter() {
    //                 let cdn_url = format!(
    //                     "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
    //                     emote
    //                 );
    //                 let start = start_emote - cur;
    //                 let expeced_location = end_emote - start_emote + 1;
    //
    //                 let (remain2, (prev, expected)) =
    //                     tuple((take(start), take(expeced_location)))(remain)?;
    //
    //                 remain = remain2;
    //                 cur = end_emote + 1;
    //
    //                 if !(prev.is_empty()
    //                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
    //                 {
    //                     result.push(MessageCDN::Normal(Normal::new(prev)));
    //                 };
    //
    //                 result.push(MessageCDN::Emote(EmoteCDN::new(emote, expected, &cdn_url)))
    //             }
    //
    //             // if remain.len() != 0 {
    //             if !remain.is_empty() {
    //                 result.push(MessageCDN::Normal(Normal::new(remain)));
    //             }
    //             Ok(("", Some(result)))
    //         }
    //     }
    // }

    // pub fn get_data_string<'a>(
    //     msg: &'a str,
    //     emotes: Option<Vec<(String, u64, u64)>>,
    //     template: &EmoteR,
    // ) -> IResult<&'a str, Option<Vec<Message>>> {
    //     match emotes {
    //         None => Ok(("", None)),
    //         Some(emoets_list) => {
    //             let mut result = vec![];
    //             let data = &template.data;
    //             let mut cur = 0;
    //             let mut remain = msg;
    //
    //             for (emote, start_emote, end_emote) in emoets_list.into_iter() {
    //                 let find_emote = data.iter().find(|value| value.id == emote);
    //                 let start = start_emote - cur;
    //                 let expeced_location = end_emote - start_emote + 1;
    //
    //                 let (remain2, (prev, expected)) =
    //                     tuple((take(start), take(expeced_location)))(remain)?;
    //
    //                 remain = remain2;
    //                 cur = end_emote + 1;
    //
    //                 if !(prev.is_empty()
    //                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
    //                 {
    //                     result.push(Message::Normal(Normal::new(prev)));
    //                 };
    //
    //                 match find_emote {
    //                     Some(value) => {
    //                         result.push(Message::Emote(value.clone()));
    //                     }
    //                     None => {
    //                         result.push(Message::Unknown(Unknown::new(expected, &emote)));
    //                     }
    //                 };
    //             }
    //
    //             if remain.len() != 0 {
    //                 result.push(Message::Normal(Normal::new(remain)));
    //             }
    //             Ok(("", Some(result)))
    //         }
    //     }
    // }

    // pub fn get_data<'a>(
    //     msg: &'a str,
    //     emotes: Option<Vec<(&str, u64, u64)>>,
    //     template: &EmotesTemplate,
    // ) -> IResult<&'a str, Option<Vec<Message>>> {
    //     match emotes {
    //         None => Ok(("", None)),
    //         Some(emoets_list) => {
    //             let mut result = vec![];
    //             let data = &template.data;
    //             let mut cur = 0;
    //             let mut remain = msg;
    //
    //             for (emote, start_emote, end_emote) in emoets_list.into_iter() {
    //                 let find_emote = data.iter().find(|value| value.id == emote);
    //                 let start = start_emote - cur;
    //                 let expeced_location = end_emote - start_emote + 1;
    //
    //                 let (remain2, (prev, expected)) =
    //                     tuple((take(start), take(expeced_location)))(remain)?;
    //
    //                 remain = remain2;
    //                 cur = end_emote + 1;
    //
    //                 if !(prev.is_empty()
    //                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
    //                 {
    //                     result.push(Message::Normal(Normal::new(prev)));
    //                 };
    //
    //                 match find_emote {
    //                     Some(value) => {
    //                         result.push(Message::Emote(value.clone()));
    //                     }
    //                     None => {
    //                         result.push(Message::Unknown(Unknown::new(expected, &emote)));
    //                     }
    //                 };
    //             }
    //
    //             if remain.len() != 0 {
    //                 result.push(Message::Normal(Normal::new(remain)));
    //             }
    //             Ok(("", Some(result)))
    //         }
    //     }
    // }
}

fn emotes_set(msg: &str) -> IResult<&str, Vec<(&str, u64, u64)>> {
    let (msg, (emote_id, locations)) = tuple((
        terminated(take_until(":"), tag(":")),
        separated_list0(tag(","), start_end),
    ))(msg)?;

    let result = locations
        .into_iter()
        .map(|(start, end)| (emote_id, start, end))
        .collect::<Vec<(&str, u64, u64)>>();

    Ok((msg, result))
}

fn emotes_set_string(msg: &str) -> IResult<&str, Vec<(String, u64, u64)>> {
    let (msg, (emote_id, locations)) = tuple((
        terminated(take_until(":"), tag(":")),
        separated_list0(tag(","), start_end),
    ))(msg)?;

    let result = locations
        .into_iter()
        .map(|(start, end)| (emote_id.to_string(), start, end))
        .collect::<Vec<(String, u64, u64)>>();

    Ok((msg, result))
}

// start , end
fn start_end(msg: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        map_res(digit1, |s: &str| s.parse::<u64>()),
        tag("-"),
        map_res(digit1, |s: &str| s.parse::<u64>()),
    )(msg)
}

// fn emotes_set_with_cdn(msg: &str) -> IResult<&str, Vec<(&str, u64, u64, &str)>> {
//     let (msg, (emote_id, locations)) = tuple((
//         terminated(take_until(":"), tag(":")),
//         separated_list0(tag(","), start_end),
//     ))(msg)?;
//
//     let result = locations
//         .into_iter()
//         .map(|(start, end)| (emote_id, start, end))
//         .collect::<Vec<(&str, u64, u64)>>();
//
//     Ok((msg, result))
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum MessageCDN {
//     Normal(Normal),
//     Emote(EmoteCDN),
//     Unknown(Unknown),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "emote")]
// pub struct EmoteCDN {
//     pub id: String,
//     pub name: String,
//     pub url: String,
// }
//
// impl EmoteCDN {
//     pub fn new<T: Into<String>>(id: T, name: T, url: T) -> EmoteCDN {
//         EmoteCDN {
//             id: id.into(),
//             name: name.into(),
//             url: url.into(),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// pub enum Message {
//     Normal(Normal),
//     Emote(Emote),
//     Unknown(Unknown),
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "normal")]
// pub struct Normal {
//     pub message: String,
// }
//
// impl Normal {
//     pub fn new<T: Into<String>>(message: T) -> Normal {
//         Normal {
//             message: message.into(),
//         }
//     }
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag = "type", rename = "unknown")]
// pub struct Unknown {
//     pub message: String,
//     pub id: String,
// }
//
// impl Unknown {
//     pub fn new<T: Into<String>>(message: T, id: T) -> Unknown {
//         Unknown {
//             message: message.into(),
//             id: id.into(),
//         }
//     }
// }
