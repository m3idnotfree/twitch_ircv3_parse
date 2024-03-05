use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};
use serde::{Deserialize, Serialize};
use twitch_highway::badges::{Badge, BadgeResponse};

use crate::tags::{BadgesTag, EmotesTag};

use super::utils::IrcMessageBase;

#[derive(Debug, PartialEq)]
pub struct PrivMsg<'a> {
    // pub tags: Option<HashMap<&'a str, &'a str>>,
    // pub badges: Option<Badges<'a>>,
    // pub emotes: Option<Emotes<'a>>,
    // pub prefix: Option<(&'a str, Option<&'a str>)>,
    pub command: &'a str,
    // pub params: ChannelNMsg,
    // pub data: IrcMessageBase<'a>,
    data: IrcMessageBase<'a>,
}

impl<'a> PrivMsg<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> PrivMsg<'a> {
        let basic = IrcMessageBase::new(tags, prefix, params);

        PrivMsg {
            // tags: basic.tags.to_owned(),
            // badges: basic.get_badges_tags(),
            // emotes: basic.get_emotes_tags(),
            // prefix: basic.prefix.to_str(),
            command: "PRIVMSG",
            // params: basic.c_m(),
            data: basic,
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_badges(&self) -> Option<BadgesTag> {
        self.data.get_badges_tags()
    }

    pub fn get_emotes(&self) -> Option<EmotesTag> {
        self.data.get_emotes_tags()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }

    pub fn output_json(&self, badge: &BadgeResponse) -> OutputMessage<String> {
        let badges = self.get_badges().map(|value| value.find_badges(badge));
        let c_m = self.get_channel_message();
        let emotes = self.get_emotes();
        let emotes = emotes
            .map(|value| value.output_message_with(&c_m.message))
            .unwrap()
            .unwrap();

        OutputMessage {
            kind: "message".into(),
            channel: c_m.channel,
            badges,
            body: emotes,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputMessage<T> {
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub badges: Option<Vec<Badge>>,
    pub body: Vec<MessageBoby<T>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageBoby<T> {
    Text(Text),
    Emote(EmoteText<T>),
    UnknownEmote(UnknownEmote),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "text")]
pub struct Text {
    pub message: String,
}

impl Text {
    pub fn new(message: String) -> Text {
        Text { message }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "emote")]
pub struct EmoteText<T> {
    pub id: String,
    pub emote: String,
    pub data: T,
}

impl<T> EmoteText<T> {
    pub fn new(id: String, emote: String, data: T) -> EmoteText<T> {
        EmoteText { id, emote, data }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename = "unknown_emote")]
pub struct UnknownEmote {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {}
