use std::collections::HashMap;

use ircv3_parse::{channel_message, Ircv3Parse};

mod badges;
pub use badges::*;

mod emotes;
pub use emotes::*;

mod utils;
pub use utils::*;

pub trait ChatFormatJson {
    fn chat_format_json(
        self,
        badges_template: &BadgeTemplate,
        emoets_template: &EmotesTemplate,
    ) -> ChatInfo;
}

#[derive(Debug, PartialEq)]
pub struct PrivMsg {
    pub tags: Option<HashMap<String, String>>,
    pub badges: Option<Vec<(String, String)>>,
    pub emotes: Option<Vec<(String, u64, u64)>>,
    pub prefix: Option<(String, Option<String>)>,
    pub command: String,
    pub params: HashMap<String, String>,
}

impl ChatFormatJson for PrivMsg {
    fn chat_format_json(
        self,
        badges_template: &BadgeTemplate,
        emoets_template: &EmotesTemplate,
    ) -> ChatInfo {
        let time = self.tags.unwrap().get("tmi-sent-ts").unwrap().to_string();
        let badges = self
            .badges
            .map(|value| Badges::get_data(value, badges_template));

        println!("{:#?}", badges);
        let m = self.params.get("message").unwrap().as_str();
        println!("m = {}", m);
        let (_, message) = Emotes::get_data(m, self.emotes, emoets_template).unwrap();

        ChatInfo::new("PRIVMSG", Some(time), badges, message)
    }
}

impl PrivMsg {
    pub fn new(
        tags: Option<HashMap<String, String>>,
        prefix: Option<(String, Option<String>)>,
        params: &str,
    ) -> PrivMsg {
        let (_, params) = channel_message(params).expect("Faieled privmsg parse params");

        let (tags, badges, emotes) = match tags {
            Some(mut value) => {
                let badges = Badges::parse(value.get("badges").unwrap().as_str());
                let emotes = Emotes::parse(value.get("emotes").unwrap().as_str());
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
            command: "PRIVMSG".to_string(),
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
pub enum Ircv3 {
    Priv(PrivMsg),
    Cap(Cap),
    Number(Number),
    Other(Other),
}
#[derive(Debug, PartialEq)]
pub struct Number {
    command: String,
    message: String,
}
impl Number {
    pub fn new<T: Into<String>>(command: T, message: T) -> Number {
        Number {
            command: command.into(),
            message: message.into(),
        }
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
pub struct Other {
    pub tags: Option<HashMap<String, String>>,
    pub prefix: Option<(String, Option<String>)>,
    pub command: String,
    pub params: String,
}

impl Other {
    pub fn new<T: Into<String>>(
        tags: Option<HashMap<String, String>>,
        prefix: Option<(String, Option<String>)>,
        command: T,
        params: T,
    ) -> Other {
        Other {
            tags,
            prefix,
            command: command.into(),
            params: params.into(),
        }
    }
}

impl ChatFormatJson for Other {
    fn chat_format_json(
        self,
        _badges_template: &BadgeTemplate,
        _emoets_template: &EmotesTemplate,
    ) -> ChatInfo {
        ChatInfo::new(self.command, None, None, None)
    }
}
#[derive(Debug, PartialEq)]
pub struct TwitchIrcMessage {}

impl TwitchIrcMessage {
    pub fn parse(msg: &str) -> Ircv3 {
        let result = Ircv3Parse::new(msg);
        match result.command.as_str() {
            "CAP" => Ircv3::Cap(Cap::new(result.message)),
            "001" => Ircv3::Number(Number::new(result.command, result.message.to_string())),
            "PRIVMSG" => Ircv3::Priv(PrivMsg::new(
                result.tags.hashmap_string(),
                result.prefix.to_string(),
                result.message,
            )),
            "CLEARCHAT" => Ircv3::Other(Other::new(
                result.tags.hashmap_string(),
                result.prefix.to_string(),
                result.command,
                result.message.to_string(),
            )),
            _ => Ircv3::Other(Other::new(
                result.tags.hashmap_string(),
                result.prefix.to_string(),
                result.command,
                result.message.to_string(),
            )),
        }
    }
}
