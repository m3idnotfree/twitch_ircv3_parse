pub mod kinds;

pub mod tags;

use ircv3_parse::Ircv3Parse;
use kinds::{
    capabilities::{
        commands::{
            ClearChat, ClearMsg, CommandReceive, GlobalUserState, HostTarget, Reconnect, RoomState,
            UserNotice, UserState, Whisper,
        },
        Cap,
    },
    Member, Notice, PrivMsg,
};

#[derive(Debug, PartialEq)]
pub enum TwitchIrcMessage<'a> {
    Privmsg(PrivMsg<'a>),
    Notice(Notice<'a>),
    /// join, part
    Member(Member),
    Command(CommandReceive<'a>),
    Cap(Cap<'a>),
    Hosttarget(HostTarget),
    Reconnect(Reconnect),
    Unknown,
    Unimplemented,
}

impl<'a> TwitchIrcMessage<'a> {
    pub fn parse(msg: &str) -> TwitchIrcMessage {
        let (_remain, result) = Ircv3Parse::parse(msg).unwrap();
        match result.command {
            "CAP" => TwitchIrcMessage::Cap(Cap::new(result.params)),
            "JOIN" | "PART" => TwitchIrcMessage::Member(Member::new(
                result.command.into(),
                result.prefix,
                result.params,
            )),
            "CLEARCHAT" | "CLEARMSG" | "GLOBALUSERSTATE" | "ROOMSTATE" | "USERNOTICE"
            | "USERSTATE" | "WHISPER" => TwitchIrcMessage::Command(CommandReceive::new(
                result.tags,
                result.command.into(),
                result.prefix,
                result.params,
            )),
            "NOTICE" => {
                TwitchIrcMessage::Notice(Notice::new(result.tags, result.prefix, result.params))
            }
            "PRIVMSG" => {
                TwitchIrcMessage::Privmsg(PrivMsg::new(result.tags, result.prefix, result.params))
            }
            "HOSTTARGET" => TwitchIrcMessage::Hosttarget(HostTarget::new(result.params)),
            "RECONNECT" => TwitchIrcMessage::Reconnect(Reconnect::new()),
            _ => TwitchIrcMessage::Unimplemented,
        }
    }
}
