use ircv3_parse::{IRCv3Params, IRCv3Prefix};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{not_line_ending, space1},
    combinator::map,
    sequence::{terminated, tuple},
    IResult,
};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct HostTarget {
    pub channel: String,
    pub hosting_channel: String,
    pub number_of_viewers: u32,
}

impl HostTarget {
    // pub fn parse(params: IRCv3Params) -> HostTarget {
    //     let channel = params.channel().unwrap().to_string();
    //     let message = params.message().unwrap().to_string();
    //     let (_, (hosting_channel, number_of_viewers)) =
    //         HostTarget::parse_hosttarget(message.as_str()).unwrap();
    //     HostTarget {
    //         channel,
    //         hosting_channel: hosting_channel.to_string(),
    //         number_of_viewers,
    //     }
    // }

    fn parse_hosttarget(msg: &str) -> IResult<&str, (&str, u32)> {
        let (msg, (hosting_channel, number_of_viewers)) = tuple((
            terminated(alt((tag("-"), take_until(" "))), space1),
            map(not_line_ending, |s: &str| s.parse::<u32>().unwrap()),
        ))(msg)?;
        Ok((msg, (hosting_channel, number_of_viewers)))
    }
}

pub fn parse_hosttarget(params: IRCv3Params) -> HostTarget {
    let channel = params.channel().unwrap().to_string();
    let message = params.message().unwrap().to_string();
    let (_, (hosting_channel, number_of_viewers)) =
        HostTarget::parse_hosttarget(message.as_str()).unwrap();
    HostTarget {
        channel: hosting_channel.to_string(),
        hosting_channel: channel.to_string(),
        number_of_viewers,
    }
}

#[derive(Debug, Default, Serialize)]
pub struct FromTo {
    from: String,
    to: String,
    message: String,
}

impl FromTo {
    pub fn parse(params: IRCv3Params, prefix: IRCv3Prefix) -> FromTo {
        // let (user, _) = self.data.get_prefix().unwrap();
        // let c_m = self.data.c_m();
        let to = prefix.server_nick().unwrap().to_string();
        let from = params.channel().unwrap().to_string();
        let message = params.message().unwrap().to_string();
        FromTo { from, to, message }
    }
}

// #[derive(Debug, Default, Serialize)]
// pub struct IsCapabilities {
//     check: bool,
// }
//
// impl IsCapabilities {
//     pub fn check(params: IRCv3Params) -> bool {
//         let middle = params.channel().unwrap();
//         let (_, ch) = IsCapabilities::is_capabi(middle).unwrap();
//         ch
//     }
//
//     fn is_capabi(msg: &str) -> IResult<&str, bool> {
//         let (cap, _) = tag("* ")(msg)?;
//
//         Ok(("", cap == "ACK"))
//     }
// }

pub fn is_capabilities(params: IRCv3Params) -> bool {
    let middle = params.channel().unwrap();
    let (_, ch) = is_capabi(middle).unwrap();
    ch
}

fn is_capabi(msg: &str) -> IResult<&str, bool> {
    let (cap, _) = tag("* ")(msg)?;

    Ok(("", cap == "ACK"))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Message {
    Normal(Normal),
    Emote(Emote),
    Unknown(Unknown),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename = "emote")]
pub struct Emote {
    pub id: String,
    pub name: String,
    pub url: String,
}

impl Emote {
    pub fn new<T: Into<String>>(id: T, name: T, url: T) -> Emote {
        Emote {
            id: id.into(),
            name: name.into(),
            url: url.into(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename = "normal")]
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

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename = "unknown")]
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
