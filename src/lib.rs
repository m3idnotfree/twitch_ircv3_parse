use ircv3_parse::{ircv3_parse, IRCv3Params, IRCv3Prefix, IRCv3Tags};
use kind::{Command, Output};

pub mod kind;
pub mod tags;
pub mod utils;

use tags::{parse_badges, parse_emotes};
pub use twitch_highway::badges::{BadgeResponse, Badges};
pub use twitch_highway::emotes::{
    Emote, EmoteChannelResponse, EmoteGlobalResponse, EmoteSetsResponse,
};

pub struct TwitchMessage<'a> {
    pub tags: IRCv3Tags<'a>,
    pub prefix: IRCv3Prefix<'a>,
    pub command: Command,
    pub params: IRCv3Params<'a>,
}

impl<'a> TwitchMessage<'a> {
    pub fn new(
        tags: IRCv3Tags<'a>,
        prefix: IRCv3Prefix<'a>,
        command: &'a str,
        params: IRCv3Params<'a>,
    ) -> Self {
        Self {
            tags,
            prefix,
            command: Command::from(command),
            params,
        }
    }

    pub fn parse(msg: &'a str) -> Self {
        let (tags, prefix, command, params) = ircv3_parse(msg);
        Self {
            tags,
            prefix,
            command: Command::from(command),
            params,
        }
    }

    pub fn output(self, badges_template: &BadgeResponse) -> Output {
        let badges = self.tags.get("badges").and_then(|value| {
            let value = parse_badges(value);
            // value.output(badges_template)
            value.output(badges_template)
        });

        let messages = self.params.message().and_then(|value| {
            self.tags.get("emotes").and_then(|k| {
                let a = parse_emotes(k);
                a.output(value)
            })
        });

        Output {
            kind: "message".to_string(),
            command: self.command,
            badges,
            body: messages,
        }
    }
}
