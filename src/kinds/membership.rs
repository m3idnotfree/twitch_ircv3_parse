use ircv3_parse::{Ircv3Params, Ircv3Prefix};

#[derive(Debug, PartialEq)]
pub struct Member {
    pub user: Option<(String, Option<String>)>,
    pub command: String,
    pub channel: String,
}

impl Member {
    pub fn new<'a>(command: String, prefix: Ircv3Prefix<'a>, params: Ircv3Params<'a>) -> Member {
        let (_, c_m) = params.channel().unwrap();
        Member {
            user: prefix.to_string(),
            channel: c_m.to_string(),
            command,
        }
    }
}
