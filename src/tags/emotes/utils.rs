// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct EmotesTag {
//     pub data: String,
// }

// impl EmotesTag {
// pub fn new<T: Into<String>>(data: T) -> EmotesTag {
//     EmotesTag { data: data.into() }
// }
// emote_id, start, end, cdn
// pub fn parse(msg: &str) -> Option<Vec<(&str, u64, u64)>> {
//     let (_, result) = opt(separated_list1(tag("/"), emotes_set))(msg).unwrap();
//     match result {
//         Some(value) => {
//             let mut value = value
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<(&str, u64, u64)>>();
//
//             value.sort_by(|a, b| a.1.cmp(&b.1));
//
//             Some(value)
//         }
//         None => None,
//     }
// }

// pub fn parse_string(msg: &str) -> Option<Vec<(String, u64, u64)>> {
//     let (_, result) = opt(separated_list1(tag("/"), emotes_set_string))(msg).unwrap();
//     match result {
//         Some(value) => {
//             let mut value = value
//                 .into_iter()
//                 .flatten()
//                 .collect::<Vec<(String, u64, u64)>>();
//
//             value.sort_by(|a, b| a.1.cmp(&b.1));
//
//             Some(value)
//         }
//         None => None,
//     }
// }

// fn find_emote_position(msg: &str, s: u64, e: u64) -> IResult<&str, (&str, &str)> {
//     tuple((take(s), take(e)))(msg)
// }

// pub fn output_message_with(&self, msg: &str) -> Option<Vec<MessageBoby<String>>> {
//     let result = EmotesTag::parse(self.data.as_str());
//
//     result.map(|value| {
//         let mut result = vec![];
//         let mut cur = 0;
//         let mut remain = msg;
//
//         for (emote, start_emote, end_emote) in value.into_iter() {
//             let start = start_emote - cur;
//             let expeced_location = end_emote - start_emote + 1;
//
//             let (remain2, (prev, expected)) =
//                 EmotesTag::find_emote_position(remain, start, expeced_location).unwrap();
//
//             remain = remain2;
//             cur = end_emote + 1;
//
//             if !(prev.is_empty()
//                 || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//             {
//                 result.push(MessageBoby::Text(Text::new(prev.into())));
//             };
//
//             result.push(MessageBoby::Emote(EmoteText::new(
//                 emote.into(),
//                 expected.to_string(),
//                 EmotesTag::cdn_url(emote),
//             )))
//         }
//
//         if !remain.is_empty() {
//             result.push(MessageBoby::Text(Text::new(remain.into())));
//         }
//         result
//     })
//     // match result {
//     //     // None => Ok(("", None)),
//     //     None => None,
//     //     Some(value) => {
//     //         let mut result = vec![];
//     //         let mut cur = 0;
//     //         let mut remain = msg;
//     //
//     //         for (emote, start_emote, end_emote) in value.into_iter() {
//     //             let start = start_emote - cur;
//     //             let expeced_location = end_emote - start_emote + 1;
//     //
//     //             // let (remain2, (prev, expected)) =
//     //             //     tuple((take(start), take(expeced_location)))(remain)?;
//     //             let (remain2, (prev, expected)) =
//     //                 EmotesTag::find_emote_position(remain, start, expeced_location).unwrap();
//     //
//     //             remain = remain2;
//     //             cur = end_emote + 1;
//     //
//     //             if !(prev.is_empty()
//     //                 || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//     //             {
//     //                 result.push(MessageBoby::Text(Text::new(prev.into())));
//     //             };
//     //
//     //             result.push(MessageBoby::Emote(EmoteText::new(
//     //                 emote.into(),
//     //                 expected.to_string(),
//     //                 EmotesTag::cdn_url(emote),
//     //             )))
//     //         }
//     //
//     //         if !remain.is_empty() {
//     //             result.push(MessageBoby::Text(Text::new(remain.into())));
//     //         }
//     //         Some(result)
//     //     }
//     // }
// }

// pub fn parse_message_with_emotes_cdn(
//     msg: &'a str,
//     emotes: Option<Vec<(String, u64, u64)>>,
// ) -> IResult<&'a str, Option<Vec<MessageCDN>>> {
//     match emotes {
//         None => Ok(("", None)),
//         Some(emotes_list) => {
//             let mut result = vec![];
//             let mut cur = 0;
//             let mut remain = msg;
//
//             for (emote, start_emote, end_emote) in emotes_list.into_iter() {
//                 let cdn_url = format!(
//                     "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
//                     emote
//                 );
//                 let start = start_emote - cur;
//                 let expeced_location = end_emote - start_emote + 1;
//
//                 let (remain2, (prev, expected)) =
//                     tuple((take(start), take(expeced_location)))(remain)?;
//
//                 remain = remain2;
//                 cur = end_emote + 1;
//
//                 if !(prev.is_empty()
//                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//                 {
//                     result.push(MessageCDN::Normal(Normal::new(prev)));
//                 };
//
//                 result.push(MessageCDN::Emote(EmoteCDN::new(
//                     emote,
//                     expected.to_string(),
//                     cdn_url,
//                 )))
//             }
//
//             // if remain.len() != 0 {
//             if !remain.is_empty() {
//                 result.push(MessageCDN::Normal(Normal::new(remain)));
//             }
//             Ok(("", Some(result)))
//         }
//     }
// }

// pub fn get_data_cdn_url(
//     msg: &'a str,
//     emotes: Option<Vec<(&str, u64, u64)>>,
// ) -> IResult<&'a str, Option<Vec<MessageCDN>>> {
//     match emotes {
//         None => Ok(("", None)),
//         Some(emotes_list) => {
//             let mut result = vec![];
//             let mut cur = 0;
//             let mut remain = msg;
//
//             for (emote, start_emote, end_emote) in emotes_list.into_iter() {
//                 let cdn_url = format!(
//                     "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
//                     emote
//                 );
//                 let start = start_emote - cur;
//                 let expeced_location = end_emote - start_emote + 1;
//
//                 let (remain2, (prev, expected)) =
//                     tuple((take(start), take(expeced_location)))(remain)?;
//
//                 remain = remain2;
//                 cur = end_emote + 1;
//
//                 if !(prev.is_empty()
//                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//                 {
//                     result.push(MessageCDN::Normal(Normal::new(prev)));
//                 };
//
//                 result.push(MessageCDN::Emote(EmoteCDN::new(emote, expected, &cdn_url)))
//             }
//
//             // if remain.len() != 0 {
//             if !remain.is_empty() {
//                 result.push(MessageCDN::Normal(Normal::new(remain)));
//             }
//             Ok(("", Some(result)))
//         }
//     }
// }

// pub fn get_data_string<'a>(
//     msg: &'a str,
//     emotes: Option<Vec<(String, u64, u64)>>,
//     template: &EmoteR,
// ) -> IResult<&'a str, Option<Vec<Message>>> {
//     match emotes {
//         None => Ok(("", None)),
//         Some(emoets_list) => {
//             let mut result = vec![];
//             let data = &template.data;
//             let mut cur = 0;
//             let mut remain = msg;
//
//             for (emote, start_emote, end_emote) in emoets_list.into_iter() {
//                 let find_emote = data.iter().find(|value| value.id == emote);
//                 let start = start_emote - cur;
//                 let expeced_location = end_emote - start_emote + 1;
//
//                 let (remain2, (prev, expected)) =
//                     tuple((take(start), take(expeced_location)))(remain)?;
//
//                 remain = remain2;
//                 cur = end_emote + 1;
//
//                 if !(prev.is_empty()
//                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//                 {
//                     result.push(Message::Normal(Normal::new(prev)));
//                 };
//
//                 match find_emote {
//                     Some(value) => {
//                         result.push(Message::Emote(value.clone()));
//                     }
//                     None => {
//                         result.push(Message::Unknown(Unknown::new(expected, &emote)));
//                     }
//                 };
//             }
//
//             if remain.len() != 0 {
//                 result.push(Message::Normal(Normal::new(remain)));
//             }
//             Ok(("", Some(result)))
//         }
//     }
// }

// pub fn get_data<'a>(
//     msg: &'a str,
//     emotes: Option<Vec<(&str, u64, u64)>>,
//     template: &EmotesTemplate,
// ) -> IResult<&'a str, Option<Vec<Message>>> {
//     match emotes {
//         None => Ok(("", None)),
//         Some(emoets_list) => {
//             let mut result = vec![];
//             let data = &template.data;
//             let mut cur = 0;
//             let mut remain = msg;
//
//             for (emote, start_emote, end_emote) in emoets_list.into_iter() {
//                 let find_emote = data.iter().find(|value| value.id == emote);
//                 let start = start_emote - cur;
//                 let expeced_location = end_emote - start_emote + 1;
//
//                 let (remain2, (prev, expected)) =
//                     tuple((take(start), take(expeced_location)))(remain)?;
//
//                 remain = remain2;
//                 cur = end_emote + 1;
//
//                 if !(prev.is_empty()
//                     || prev.len() == 1 && prev.chars().next().unwrap().is_whitespace())
//                 {
//                     result.push(Message::Normal(Normal::new(prev)));
//                 };
//
//                 match find_emote {
//                     Some(value) => {
//                         result.push(Message::Emote(value.clone()));
//                     }
//                     None => {
//                         result.push(Message::Unknown(Unknown::new(expected, &emote)));
//                     }
//                 };
//             }
//
//             if remain.len() != 0 {
//                 result.push(Message::Normal(Normal::new(remain)));
//             }
//             Ok(("", Some(result)))
//         }
//     }
// }
// }