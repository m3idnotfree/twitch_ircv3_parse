mod privmsg;
use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};
pub use privmsg::*;

pub mod utils;

pub mod capabilities;

mod notice;
pub use notice::Notice;

mod membership;
pub use membership::Member;

use self::utils::IrcMessageBase;

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
pub struct Unknown<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> Unknown<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        command: &'a str,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> Unknown<'a> {
        let basic = IrcMessageBase::new(tags, prefix, params);

        Unknown {
            command,
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
