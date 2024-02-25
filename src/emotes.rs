use nom::{
    bytes::complete::{tag, take, take_until},
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::{separated_list0, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Emotes {}

impl Emotes {
    pub fn parse(msg: &str) -> Option<Vec<(String, u64, u64)>> {
        let (_, result) = opt(separated_list1(tag("/"), emotes_set))(msg).unwrap();
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

    pub fn get_data<'a>(
        msg: &'a str,
        emotes: Option<Vec<(String, u64, u64)>>,
        template: &EmotesTemplate,
    ) -> IResult<&'a str, Vec<Message>> {
        match emotes {
            None => Ok(("", vec![Message::Normal(Normal::new(msg))])),
            Some(emoets_list) => {
                let mut result = vec![];
                let data = &template.data;
                let mut cur = 0;
                let mut remain = msg;

                for (emote, start, end) in emoets_list.into_iter() {
                    let find_emote = data.iter().find(|value| value.id == emote);
                    let start = start - cur;
                    let expeced_location = end - start + 1;

                    let (remain2, (prev, expected)) =
                        tuple((take(start), take(expeced_location)))(remain)?;

                    remain = remain2;
                    cur = end + 1;

                    result.push(Message::Normal(Normal::new(prev)));

                    match find_emote {
                        Some(value) => {
                            result.push(Message::Emote(value.clone()));
                        }
                        None => {
                            result.push(Message::Unknown(Unknown::new(expected, &emote)));
                        }
                    }
                }

                result.push(Message::Normal(Normal::new(remain)));
                Ok(("", result))
            }
        }
    }
}

fn emotes_set(msg: &str) -> IResult<&str, Vec<(String, u64, u64)>> {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub images: Images,
    pub format: Vec<String>,
    pub scale: Vec<String>,
    pub theme_mode: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub url_1x: String,
    pub url_2x: String,
    pub url_4x: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmotesTemplate {
    data: Vec<Emote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Normal(Normal),
    Emote(Emote),
    Unknown(Unknown),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Normal {
    pub message: String,
}

impl Normal {
    pub fn new<T: Into<String>>(message: T) -> Normal {
        Normal {
            message: message.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unknown {
    pub message: String,
    pub id: String,
}

impl Unknown {
    pub fn new<T: Into<String>>(message: T, id: T) -> Unknown {
        Unknown {
            message: message.into(),
            id: id.into(),
        }
    }
}
