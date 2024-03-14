use std::fmt;

use serde::Serialize;
use twitch_highway::badges::Badge;

use crate::utils::Message;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Command {
    PrivMsg,
    Notice,
    Join,
    Part,
    ClearChat,
    ClearMsg,
    GlobalUserState,
    RoomState,
    UserNotice,
    UserState,
    Cap,
    HostTarget,
    Reconnect,
    Whisper,
    Unknown,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        match value {
            "CAP" => Command::Cap,
            "JOIN" => Command::Join,
            "PART" => Command::Part,
            "GLOBALUSERSTATE" => Command::GlobalUserState,
            "ROOMSTATE" => Command::RoomState,
            "USERNOTICE" => Command::UserNotice,
            "USERSTATE" => Command::UserState,
            "WHISPER" => Command::Whisper,
            "CLEARCHAT" => Command::ClearChat,
            "CLEARMSG" => Command::ClearMsg,
            "NOTICE" => Command::Notice,
            "PRIVMSG" => Command::PrivMsg,
            "HOSTTARGET" => Command::HostTarget,
            "RECONNECT" => Command::Reconnect,
            _ => Command::Unknown,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Cap => write!(f, "CAP"),
            Command::Join => write!(f, "JOIN"),
            Command::Part => write!(f, "PART"),
            Command::GlobalUserState => write!(f, "GLOBALUSERSTATE"),
            Command::RoomState => write!(f, "ROOMSTATE"),
            Command::UserNotice => write!(f, "USERNOTICE"),
            Command::UserState => write!(f, "USERSTATE"),
            Command::Whisper => write!(f, "WHISPER"),
            Command::ClearChat => write!(f, "CLEARCHAT"),
            Command::ClearMsg => write!(f, "CLEARMSG"),
            Command::Notice => write!(f, "NOTICE"),
            Command::PrivMsg => write!(f, "PRIVMSG"),
            Command::HostTarget => write!(f, "HOSTTARGET"),
            Command::Reconnect => write!(f, "RECONNECT"),
            Command::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl From<Command> for String {
    fn from(val: Command) -> Self {
        match val {
            Command::Cap => "CAP".to_string(),
            Command::Join => "JOIN".to_string(),
            Command::Part => "PART".to_string(),
            Command::GlobalUserState => "GLOBALUSERSTATE".to_string(),
            Command::RoomState => "ROOMSTATE".to_string(),
            Command::UserNotice => "USERNOTICE".to_string(),
            Command::UserState => "USERSTATE".to_string(),
            Command::Whisper => "WHISPER".to_string(),
            Command::ClearChat => "CLEARCHAT".to_string(),
            Command::ClearMsg => "CLEARMSG".to_string(),
            Command::Notice => "NOTICE".to_string(),
            Command::PrivMsg => "PRIVMSG".to_string(),
            Command::HostTarget => "HOSTTARGET".to_string(),
            Command::Reconnect => "RECONNECT".to_string(),
            Command::Unknown => "UNKNOWN".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Output {
    #[serde(rename = "type")]
    pub kind: String,
    pub command: Command,
    pub badges: Option<Vec<Badge>>,
    pub body: Option<Vec<Message>>,
}
