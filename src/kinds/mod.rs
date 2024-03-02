mod privmsg;
pub use privmsg::PrivMsg;

pub mod utils;

pub mod capabilities;

mod notice;
pub use notice::Notice;

mod membership;
pub use membership::Member;

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
