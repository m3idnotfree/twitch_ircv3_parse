use nom::{
    bytes::complete::{is_not, tag, take_until1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use serde::{Deserialize, Serialize};
use twitch_helix_api::badges::{Badge, BadgeResponse};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BadgesTag<'a> {
    pub data: &'a str,
}

impl<'a> BadgesTag<'a> {
    pub fn new(data: &'a str) -> BadgesTag {
        BadgesTag { data }
    }

    pub fn parse_string(&self) -> Option<Vec<(String, String)>> {
        let (_, result) = badges_string(self.data).unwrap();

        result
    }

    pub fn parse(&self) -> Option<Vec<(&str, &str)>> {
        let (_, result) = badges_str(self.data).unwrap();

        result
    }

    pub fn get_data_string(badges: Vec<(String, String)>, template: &BadgeResponse) -> Vec<Badge> {
        let data = &template.data;
        badges
            .into_iter()
            .map(|(key, value)| {
                let select = data.iter().find(|b| b.set_id == key).unwrap();

                let versions = select.versions.iter().find(|h| h.id == value).unwrap();
                Badge {
                    set_id: key,
                    versions: versions.clone(),
                }
            })
            .collect()
    }

    pub fn get_data(&self, template: &BadgeResponse) -> Vec<Badge> {
        let badges = self.parse().unwrap();
        let data = &template.data;
        badges
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
    }
}

fn badges_string(msg: &str) -> IResult<&str, Option<Vec<(String, String)>>> {
    opt(separated_list1(tag(","), key_value_string))(msg)
}

fn badges_str(msg: &str) -> IResult<&str, Option<Vec<(&str, &str)>>> {
    opt(separated_list1(tag(","), key_value_str))(msg)
}

fn key_value_string(msg: &str) -> IResult<&str, (String, String)> {
    separated_pair(
        map(take_until1("/"), |s: &str| s.to_string()),
        tag("/"),
        map(is_not(","), |s: &str| s.to_string()),
    )(msg)
}

fn key_value_str(msg: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(take_until1("/"), tag("/"), is_not(","))(msg)
}
