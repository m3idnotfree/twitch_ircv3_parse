use nom::{
    bytes::complete::{is_not, tag, take_until1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use serde::Serialize;
use twitch_highway::badges::{Badge, BadgeResponse};

#[derive(Debug, PartialEq, Serialize)]
pub struct BadgesTag(pub Option<Vec<(String, String)>>);

impl BadgesTag {
    pub fn new(badges: Option<Vec<(String, String)>>) -> Self {
        Self(badges)
    }

    pub fn output(self, template: &BadgeResponse) -> Option<Vec<Badge>> {
        self.0.map(|value| {
            let data = &template.data;
            value
                .into_iter()
                .map(|(key, value)| {
                    let select = data.iter().find(|b| b.set_id == key).unwrap();

                    let versions = select.versions.iter().find(|h| h.id == value).unwrap();
                    Badge {
                        set_id: key.to_string(),
                        versions: versions.clone(),
                    }
                })
                .collect()
        })
    }
}

impl AsRef<Option<Vec<(String, String)>>> for BadgesTag {
    fn as_ref(&self) -> &Option<Vec<(String, String)>> {
        &self.0
    }
}

pub fn parse_badges(msg: &str) -> BadgesTag {
    let (_, result) = parse(msg).unwrap();

    BadgesTag(result)
}

fn parse(msg: &str) -> IResult<&str, Option<Vec<(String, String)>>> {
    opt(separated_list1(tag(","), key_value))(msg)
}

fn key_value(msg: &str) -> IResult<&str, (String, String)> {
    separated_pair(
        // take_until1("/"),
        map(take_until1("/"), |s: &str| s.to_string()),
        tag("/"),
        // is_not(",")
        map(is_not(","), |s: &str| s.to_string()),
    )(msg)
}
