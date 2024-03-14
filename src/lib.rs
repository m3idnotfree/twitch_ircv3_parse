pub mod kinds;

mod tags;
pub use tags::*;

pub mod utils;

pub use twitch_highway::badges::{BadgeResponse, Badges};
pub use twitch_highway::emotes::{
    Emote, EmoteChannelResponse, EmoteGlobalResponse, EmoteSetsResponse,
};

// use kinds::{
//     capabilities::{
//         commands::{
//             ClearChat, ClearMsg, GlobalUserState, HostTarget, Reconnect, RoomState, UserNotice,
//             UserState, Whisper,
//         },
//         Cap,
//     },
//     Member, Notice, PrivMsg, Unknown,
// };

// #[derive(Debug, PartialEq)]
// pub enum TwitchIrcMessage<'a> {
//     Privmsg(PrivMsg<'a>),
//     Notice(Notice<'a>),
//     /// join, part
//     Member(Member),
//     ClearChat(ClearChat<'a>),
//     ClearMsg(ClearMsg<'a>),
//     GlobalUserState(GlobalUserState<'a>),
//     RoomState(RoomState<'a>),
//     UserNotice(UserNotice<'a>),
//     UserState(UserState<'a>),
//     Cap(Cap<'a>),
//     Hosttarget(HostTarget),
//     Reconnect(Reconnect),
//     Whisper(Whisper<'a>),
//     Unknown(Unknown<'a>),
//     Unimplemented,
// }

// impl<'a> TwitchIrcMessage<'a> {
//     pub fn parse(msg: &str) -> TwitchIrcMessage {
//         let (tags, prefix, command, params) = ircv3_parse(msg);
//         match command {
//             "CAP" => TwitchIrcMessage::Cap(Cap::new(params)),
//             "JOIN" | "PART" => {
//                 TwitchIrcMessage::Member(Member::new(command.into(), prefix, params))
//             }
//             "GLOBALUSERSTATE" => {
//                 TwitchIrcMessage::GlobalUserState(GlobalUserState::new(tags, prefix, params))
//             }
//
//             "ROOMSTATE" => TwitchIrcMessage::RoomState(RoomState::new(tags, prefix, params)),
//             "USERNOTICE" => TwitchIrcMessage::UserNotice(UserNotice::new(tags, prefix, params)),
//             "USERSTATE" => TwitchIrcMessage::UserState(UserState::new(tags, prefix, params)),
//             "WHISPER" => TwitchIrcMessage::Whisper(Whisper::new(tags, prefix, params)),
//             "CLEARCHAT" => TwitchIrcMessage::ClearChat(ClearChat::new(tags, prefix, params)),
//             "CLEARMSG" => TwitchIrcMessage::ClearMsg(ClearMsg::new(tags, prefix, params)),
//             "NOTICE" => TwitchIrcMessage::Notice(Notice::new(tags, prefix, params)),
//             "PRIVMSG" => TwitchIrcMessage::Privmsg(PrivMsg::new(tags, prefix, params)),
//             "HOSTTARGET" => TwitchIrcMessage::Hosttarget(HostTarget::new(params)),
//             "RECONNECT" => TwitchIrcMessage::Reconnect(Reconnect::default()),
//             _ => TwitchIrcMessage::Unknown(Unknown::new(tags, command, prefix, params)),
//         }
//     }
// }
