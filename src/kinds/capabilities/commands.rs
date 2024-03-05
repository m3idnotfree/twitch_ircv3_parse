use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{not_line_ending, space1},
    sequence::{terminated, tuple},
    IResult,
};
use serde::{Deserialize, Serialize};

use crate::kinds::utils::IrcMessageBase;

#[derive(Debug, PartialEq)]
pub struct CommandReceive<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> CommandReceive<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        command: &'a str,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> CommandReceive<'a> {
        CommandReceive {
            command,
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClearChat<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> ClearChat<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> ClearChat<'a> {
        ClearChat {
            command: "CLEARCHAT",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel_n_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }

    pub fn get_channel(&self) -> &str {
        self.data.channel()
    }
}
#[derive(Debug, PartialEq)]
pub struct ClearMsg<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> ClearMsg<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> ClearMsg<'a> {
        ClearMsg {
            command: "CLEARMSG",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel_n_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }
}

#[derive(Debug, PartialEq)]
pub struct GlobalUserState<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> GlobalUserState<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> GlobalUserState<'a> {
        let basic = IrcMessageBase::new(tags, prefix, params);

        GlobalUserState {
            command: "GLOBALUSERSTATE",
            data: basic,
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }
}

#[derive(Debug, PartialEq)]
pub struct HostTarget {
    pub command: String,
    pub host_channel: String,
    pub channel: String,
    pub number_of_viewers: String,
}

impl HostTarget {
    pub fn new(params: Ircv3Params<'_>) -> HostTarget {
        let (_, c_m) = params.channel_n_message().unwrap();
        let binding = c_m.to_owned();
        let (_, (host, num)) = HostTarget::parse_params(binding.message.as_str()).unwrap();
        HostTarget {
            command: "HOSTTARGET".into(),
            channel: host.into(),
            host_channel: c_m.channel,
            number_of_viewers: num.into(),
        }
    }

    // channel, number_of_veiwers
    pub fn parse_params(msg: &str) -> IResult<&str, (&str, &str)> {
        let (msg, (host, number_of_viewers)) = tuple((
            terminated(alt((tag("-"), take_until(" "))), space1),
            not_line_ending,
        ))(msg)?;
        Ok((msg, (host, number_of_viewers)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Reconnect {
    pub command: String,
}

impl Default for Reconnect {
    fn default() -> Self {
        Self {
            command: "RECONNECT".into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RoomState<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> RoomState<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> RoomState<'a> {
        RoomState {
            command: "ROOMSTATE",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel(&self) -> &str {
        self.data.channel()
    }
}

#[derive(Debug, PartialEq)]
pub struct UserNotice<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> UserNotice<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> UserNotice<'a> {
        UserNotice {
            command: "USERNOTICE",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.data.get_prefix()
    }

    pub fn get_channel_n_message(&self) -> ChannelnMsg {
        self.data.c_m()
    }

    pub fn get_channel(&self) -> &str {
        self.data.channel()
    }
}

#[derive(Debug, PartialEq)]
pub struct UserState<'a> {
    pub command: &'a str,
    data: IrcMessageBase<'a>,
}

impl<'a> UserState<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> UserState<'a> {
        UserState {
            command: "USERSTATE",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn get_tags(&self) -> Option<HashMap<String, String>> {
        self.data.tags.clone()
    }

    pub fn get_channel(&self) -> ChannelnMsg {
        self.data.c_m()
    }
}

#[derive(Debug, PartialEq)]
pub struct Whisper<'a> {
    pub command: &'a str,
    pub data: IrcMessageBase<'a>,
}

impl<'a> Whisper<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> Whisper<'a> {
        Whisper {
            command: "WHISPER",
            data: IrcMessageBase::new(tags, prefix, params),
        }
    }

    pub fn from_to(&self) -> FromTo {
        let (user, _) = self.data.get_prefix().unwrap();
        let c_m = self.data.c_m();
        FromTo {
            from: c_m.channel,
            to: user.into(),
            message: c_m.message,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromTo {
    from: String,
    to: String,
    message: String,
}
