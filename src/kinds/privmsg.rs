use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};

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

    pub fn get_tags(&self) -> Option<HashMap<&'a str, &'a str>> {
        self.data.get_tags()
    }

    pub fn get_badges(&self) -> Option<BadgesTag<'a>> {
        self.data.get_badges_tags()
    }

    pub fn get_emotes(&self) -> Option<EmotesTag<'a>> {
        self.data.get_emotes_tags()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }
    pub fn get_channel_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }
}
