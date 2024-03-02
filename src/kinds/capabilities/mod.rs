pub mod commands;

use ircv3_parse::Ircv3Params;
use nom::{bytes::complete::tag, IResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cap<'a> {
    pub command: &'a str,
    pub is_capabilities: bool,
}

impl<'a> Cap<'a> {
    pub fn new(params: Ircv3Params<'a>) -> Cap {
        let (_, middle) = params.middle_n_message().unwrap();
        let (_, is_capabilities) = Cap::is_capabilities(&middle.middle).unwrap();

        Cap {
            command: "CAP",
            is_capabilities,
        }
    }

    fn is_capabilities(msg: &str) -> IResult<&str, bool> {
        let (cap, _) = tag("* ")(msg)?;

        Ok(("", cap == "ACK"))
    }
}
