use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};

use crate::kinds::utils::IrcMessageBase;

#[derive(Debug, PartialEq)]
pub struct Notice<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> Notice<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> Notice<'a> {
        let basic = IrcMessageBase::new(tags, prefix, params);

        Notice {
            command: "NOTICE",
            data: basic,
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }
}
