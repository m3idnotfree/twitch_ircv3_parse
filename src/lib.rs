pub mod kinds;

pub mod tags;

pub use twitch_highway::badges::{BadgeResponse, Badges};
pub use twitch_highway::emotes::{
    Emote, EmoteChannelResponse, EmoteGlobalResponse, EmoteSetsResponse,
};

use ircv3_parse::Ircv3Parse;
use kinds::{
    capabilities::{
        commands::{
            ClearChat, ClearMsg, GlobalUserState, HostTarget, Reconnect, RoomState, UserNotice,
            UserState, Whisper,
        },
        Cap,
    },
    Member, Notice, PrivMsg, Unknown,
};

#[derive(Debug, PartialEq)]
pub enum TwitchIrcMessage<'a> {
    Privmsg(PrivMsg<'a>),
    Notice(Notice<'a>),
    /// join, part
    Member(Member),
    ClearChat(ClearChat<'a>),
    ClearMsg(ClearMsg<'a>),
    GlobalUserState(GlobalUserState<'a>),
    RoomState(RoomState<'a>),
    UserNotice(UserNotice<'a>),
    UserState(UserState<'a>),
    Cap(Cap<'a>),
    Hosttarget(HostTarget),
    Reconnect(Reconnect),
    Whisper(Whisper<'a>),
    Unknown(Unknown<'a>),
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
            "GLOBALUSERSTATE" => TwitchIrcMessage::GlobalUserState(GlobalUserState::new(
                result.tags,
                result.prefix,
                result.params,
            )),

            "ROOMSTATE" => TwitchIrcMessage::RoomState(RoomState::new(
                result.tags,
                result.prefix,
                result.params,
            )),
            "USERNOTICE" => TwitchIrcMessage::UserNotice(UserNotice::new(
                result.tags,
                result.prefix,
                result.params,
            )),
            "USERSTATE" => TwitchIrcMessage::UserState(UserState::new(
                result.tags,
                result.prefix,
                result.params,
            )),
            "WHISPER" => {
                TwitchIrcMessage::Whisper(Whisper::new(result.tags, result.prefix, result.params))
            }
            "CLEARCHAT" => TwitchIrcMessage::ClearChat(ClearChat::new(
                result.tags,
                result.prefix,
                result.params,
            )),
            "CLEARMSG" => {
                TwitchIrcMessage::ClearMsg(ClearMsg::new(result.tags, result.prefix, result.params))
            }
            "NOTICE" => {
                TwitchIrcMessage::Notice(Notice::new(result.tags, result.prefix, result.params))
            }
            "PRIVMSG" => {
                TwitchIrcMessage::Privmsg(PrivMsg::new(result.tags, result.prefix, result.params))
            }
            "HOSTTARGET" => TwitchIrcMessage::Hosttarget(HostTarget::new(result.params)),
            "RECONNECT" => TwitchIrcMessage::Reconnect(Reconnect::default()),
            _ => TwitchIrcMessage::Unknown(Unknown::new(
                result.tags,
                result.command,
                result.prefix,
                result.params,
            )),
        }
    }
}
