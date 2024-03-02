use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};

use crate::tags::{BadgesTag, EmotesTag};

#[derive(Debug, PartialEq)]
pub struct IrcMessageBase<'a> {
    tags: Option<HashMap<&'a str, &'a str>>,
    prefix: Ircv3Prefix<'a>,
    params: Ircv3Params<'a>,
}

pub trait IrcMessageTrait {
    fn get_channel_message(&self) -> ChannelnMsg;
    fn get_channel(&self) -> &str;
    fn get_tags(&self) -> Option<HashMap<&str, &str>>;
    fn get_prefix(&self) -> Option<(&str, Option<&str>)>;
}

impl<'a> IrcMessageBase<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> IrcMessageBase<'a> {
        let tags = tags.to_hashmap_str();

        IrcMessageBase {
            tags,
            prefix,
            params,
        }
    }

    pub fn get_badges_tags(&self) -> Option<BadgesTag<'a>> {
        self.tags.as_ref().map(|value| {
            let result = value.get("badges").unwrap().to_owned();

            BadgesTag::new(result)
        })
    }

    pub fn get_emotes_tags(&self) -> Option<EmotesTag<'a>> {
        self.tags.as_ref().map(|value| {
            let result = value.get("emotes").unwrap().to_owned();
            EmotesTag::new(result)
        })
    }

    /// notice
    pub fn get_msg_id_tags(&self) -> Option<String> {
        self.tags.as_ref().map(|value| {
            let result = value.get("msg-id").unwrap().to_owned();

            result.to_string()
        })
    }
    /// replay to a chat message
    pub fn get_id_tags(&self) -> Option<String> {
        self.tags.as_ref().map(|value| {
            let result = value.get("id").unwrap().to_owned();

            result.to_string()
        })
    }
    pub fn c_m(&self) -> ChannelnMsg {
        let (_, channel_message) = self.params.channel_n_message().unwrap();

        channel_message
    }

    pub fn channel(&self) -> &str {
        let (_, channel) = self.params.channel().unwrap();
        channel
    }

    pub fn get_tags(&self) -> Option<HashMap<&'a str, &'a str>> {
        self.tags.to_owned()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.prefix.to_str()
    }
}
