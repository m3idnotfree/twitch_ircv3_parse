use std::collections::HashMap;

use ircv3_parse::{ChannelnMsg, Ircv3Params, Ircv3Prefix, Ircv3TagsParse};
use serde::{Deserialize, Serialize};

use crate::tags::{BadgesTag, EmotesTag};

#[derive(Debug, PartialEq)]
pub struct IrcMessageBase<'a> {
    // tags: Option<HashMap<&'a str, &'a str>>,
    pub tags: Option<HashMap<String, String>>,
    // tags: Ircv3TagsParse<'a>,
    prefix: Ircv3Prefix<'a>,
    params: Ircv3Params<'a>,
}

// pub trait IrcMessageTrait {
//     fn get_channel_message(&self) -> ChannelnMsg;
//     fn get_channel(&self) -> &str;
//     fn get_tags(&self) -> Option<HashMap<&str, &str>>;
//     fn get_prefix(&self) -> Option<(&str, Option<&str>)>;
// }

impl<'a> IrcMessageBase<'a> {
    pub fn new(
        tags: Ircv3TagsParse<'a>,
        prefix: Ircv3Prefix<'a>,
        params: Ircv3Params<'a>,
    ) -> IrcMessageBase<'a> {
        // let ta = tags.to_hashmap_str().clone();
        // let tag = tags.to_hashmap_string().to_owned();

        IrcMessageBase {
            tags: tags.to_hashmap_string(),
            prefix,
            params,
        }
    }

    pub fn get_badges_tags(&self) -> Option<BadgesTag> {
        let result = self.tags.as_ref().map(|value| {
            let result = value.get("badges").unwrap();

            BadgesTag::new(result)
        });
        result
    }

    pub fn get_emotes_tags(&self) -> Option<EmotesTag> {
        // self.tags.clone().as_ref().map(|value| {
        self.tags.as_ref().map(|value| {
            let result = value.get("emotes").unwrap();
            EmotesTag::new(result)
        })
    }

    /// notice
    pub fn get_msg_id_tags(&self) -> Option<String> {
        self.tags.as_ref().map(|value| {
            let result = value.get("msg-id").unwrap();

            result.clone()
        })
    }
    /// replay to a chat message
    pub fn get_id_tags(&self) -> Option<String> {
        self.tags.as_ref().map(|value| {
            let result = value.get("id").unwrap();

            result.clone()
        })
    }

    pub fn c_m(&self) -> ChannelnMsg {
        let (_, channel_message) = self.params.channel_n_message().unwrap();

        channel_message
    }

    pub fn channel(&self) -> &str {
        let (_, channel) = self.params.channel().unwrap();
        channel
    }

    // pub fn get_tags(&self) -> Option<HashMap<&'a str, &'a str>> {
    //     // self.tags.to_hashmap_str().to_owned()
    //     self.tags..to_owned()
    // }

    pub fn get_prefix(&self) -> Option<(&'a str, Option<&'a str>)> {
        self.prefix.to_str()
    }

    pub fn output_json(&self, command: &str) -> OutputDefaultJson {
        // let tags = self.get_tags().into_iter().map(|(k, v)| (k, v)).collect();
        // let tags = self.tags.to_owned().into_iter().map(|v| v.get).collect();
        let prefix = self.prefix.to_string();
        let command = command.to_string();
        // let (_, params) = self.params.channel_n_message().unwrap();
        OutputDefaultJson {
            tags: self.tags.clone(),
            prefix,
            command,
            // params,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputDefaultJson {
    tags: Option<HashMap<String, String>>,
    prefix: Option<(String, Option<String>)>,
    command: String,
    // params: String,
}
