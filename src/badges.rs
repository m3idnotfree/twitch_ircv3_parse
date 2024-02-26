use nom::{
    bytes::complete::{is_not, tag, take_until1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Badges {}

impl Badges {
    pub fn parse(msg: &str) -> Option<Vec<(String, String)>> {
        let (_, result) = badges(msg).unwrap();
        result
    }

    pub fn get_data(badges: Vec<(String, String)>, template: &BadgeTemplate) -> Vec<BadgeData> {
        let data = &template.data;
        badges
            .into_iter()
            .map(|(key, value)| {
                let select = data.iter().find(|b| b.set_id == key).unwrap();

                let versions = select.versions.iter().find(|h| h.id == value).unwrap();
                BadgeData {
                    set_id: key,
                    versions: versions.clone(),
                }
            })
            .collect()
    }
}

fn badges(msg: &str) -> IResult<&str, Option<Vec<(String, String)>>> {
    opt(separated_list1(tag(","), key_value))(msg)
}

fn key_value(msg: &str) -> IResult<&str, (String, String)> {
    separated_pair(
        map(take_until1("/"), |s: &str| s.to_string()),
        tag("/"),
        map(is_not(","), |s: &str| s.to_string()),
    )(msg)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeData {
    pub set_id: String,
    pub versions: Versions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadgeDataJson {
    pub set_id: String,
    pub versions: Vec<Versions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Versions {
    pub click_action: Option<String>,
    pub click_url: Option<String>,
    pub description: String,
    pub id: String,
    pub image_url_1x: String,
    pub image_url_2x: String,
    pub image_url_4x: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BadgeTemplate {
    data: Vec<BadgeDataJson>,
}