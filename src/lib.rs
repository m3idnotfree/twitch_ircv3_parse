use std::collections::HashMap;

use ircv3_parse::{channel_message_str, Ircv3Parse};

mod badges;
pub use badges::*;

mod emotes;
pub use emotes::*;

mod utils;
use serde::{Deserialize, Serialize};
pub use utils::*;

pub trait ChatFormatJson<'a> {
    fn chat_format_json(
        self,
        badges_template: &BadgeTemplate,
        emoets_template: &EmotesTemplate,
    ) -> ChatInfo<'a>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivMsg<'a> {
    pub tags: Option<HashMap<&'a str, &'a str>>,
    pub badges: Option<Vec<(&'a str, &'a str)>>,
    pub emotes: Option<Vec<(&'a str, u64, u64)>>,
    pub prefix: Option<(&'a str, Option<&'a str>)>,
    pub command: &'a str,
    pub params: HashMap<&'a str, &'a str>,
}

impl<'a> ChatFormatJson<'a> for PrivMsg<'a> {
    fn chat_format_json(
        self,
        badges_template: &BadgeTemplate,
        emoets_template: &EmotesTemplate,
    ) -> ChatInfo<'a> {
        let binding = self.tags.unwrap();

        let time = binding.get("tmi-sent-ts").unwrap();
        let badges = self
            .badges
            .map(|value| Badges::get_data(value, badges_template));

        let m = self.params.get("message").unwrap();

        let (_, message) = Emotes::get_data(m, self.emotes, emoets_template).unwrap();

        ChatInfo::new("PRIVMSG", Some(time), badges, message)
    }
}

impl<'a> PrivMsg<'a> {
    pub fn new(
        tags: Option<HashMap<&'a str, &'a str>>,
        prefix: Option<(&'a str, Option<&'a str>)>,
        params: &'a str,
    ) -> PrivMsg<'a> {
        let (_, params) = channel_message_str(params).expect("Faieled privmsg parse params");

        let (tags, badges, emotes) = match tags {
            Some(mut value) => {
                let badges = Badges::parse(value.get("badges").unwrap());
                let emotes = Emotes::parse(value.get("emotes").unwrap());

                value.remove("badges");
                value.remove("emotes");

                (Some(value), badges, emotes)
            }
            None => (None, None, None),
        };
        PrivMsg {
            tags,
            badges,
            emotes,
            prefix,
            command: "PRIVMSG",
            params,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Join,
    Part,
    Privmsg,
    Clearchat,
    Clearmsg,
    Globaluserstate,
    Hosttarget,
    Notice,
    Reconnect,
    Roomstate,
    Usernotice,
    Userstate,
    Whisper,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Ircv3<'a> {
    Priv(PrivMsg<'a>),
    Cap(Cap),
    Number(Number<'a>),
    Other(Other<'a>),
}
#[derive(Debug, PartialEq)]
pub struct Number<'a> {
    command: &'a str,
    message: &'a str,
}
impl<'a> Number<'a> {
    pub fn new(command: &'a str, message: &'a str) -> Number<'a> {
        Number { command, message }
    }
}

#[derive(Debug, PartialEq)]
pub struct Cap {
    is_capabilities: bool,
}

impl Cap {
    pub fn new(params: &str) -> Cap {
        let (_, is_capabilities) = is_capabilities(params).unwrap();

        Cap { is_capabilities }
    }
}

#[derive(Debug, PartialEq)]
pub struct Other<'a> {
    pub tags: Option<HashMap<&'a str, &'a str>>,
    pub prefix: Option<(&'a str, Option<&'a str>)>,
    pub command: &'a str,
    pub params: &'a str,
}

impl<'a> Other<'a> {
    pub fn new(
        tags: Option<HashMap<&'a str, &'a str>>,
        prefix: Option<(&'a str, Option<&'a str>)>,
        command: &'a str,
        params: &'a str,
    ) -> Other<'a> {
        Other {
            tags,
            prefix,
            command,
            params,
        }
    }
}

impl<'a> ChatFormatJson<'a> for Other<'a> {
    fn chat_format_json(
        self,
        _badges_template: &BadgeTemplate,
        _emoets_template: &EmotesTemplate,
    ) -> ChatInfo<'a> {
        ChatInfo::new(self.command, None, None, None)
    }
}
#[derive(Debug, PartialEq)]
pub struct TwitchIrcMessage {}

impl TwitchIrcMessage {
    pub fn parse(msg: &str) -> Ircv3 {
        let result = Ircv3Parse::new(msg);
        match result.command {
            "CAP" => Ircv3::Cap(Cap::new(result.message)),
            "001" => Ircv3::Number(Number::new("001", result.message)),
            "PRIVMSG" => Ircv3::Priv(PrivMsg::new(
                result.tags.hashmap_str(),
                result.prefix.to_str(),
                result.message,
            )),
            "CLEARCHAT" => Ircv3::Other(Other::new(
                result.tags.hashmap_str(),
                result.prefix.to_str(),
                result.command,
                result.message,
            )),
            _ => Ircv3::Other(Other::new(
                result.tags.hashmap_str(),
                result.prefix.to_str(),
                result.command,
                result.message,
            )),
        }
    }
}
