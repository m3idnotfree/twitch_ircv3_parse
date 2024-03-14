use twitch_ircv3_parse::{tags::parse_emotes, TwitchMessage};

#[test]
fn priv_base_unit_rn() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let msg = TwitchMessage::parse(msg);
    assert_eq!(msg.command.to_string(), "PRIVMSG");
    assert_eq!(msg.prefix.server_nick(), Some("foo"));
    assert_eq!(msg.prefix.user(), Some("foo@foo.tmi.twitch.tv"));
    assert_eq!(msg.params.channel(), Some("#bar"));
    assert_eq!(msg.params.message(), Some("bleedPurple"));
}

#[test]
fn privmsg_chat_room_unit() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "PRIVMSG");
    assert_eq!(msg.tags.get("tmi-sent-ts"), Some("1507246572675"));
    let emotes = msg.tags.get("emotes");
    assert_eq!(emotes, Some("25:0-4,12-16/1902:6-10"));
    let emotes = parse_emotes(emotes.unwrap());

    let expected_tags = vec![
        ("25".to_string(), 0, 4),
        ("1902".to_string(), 6, 10),
        ("25".to_string(), 12, 16),
    ];

    assert_eq!(emotes.as_ref(), &Some(expected_tags))
}

// #[test]
// fn priv_postform() {
//     let badges_json = fs::read_to_string("badges.json").unwrap();
//     let badges_json: BadgeResponse = serde_json::from_str(&badges_json).unwrap();
//
//     let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n";
//     let msg = Ircv3Parse::new(msg);
//     let expected = PrivMsg::new(msg.tags, msg.prefix, msg.params);
//     let expected = expected.output_json(&badges_json);
//
//     let expected_badges = vec![Badge {
//         set_id: "turbo".into(),
//         versions: Versions {
//             id: "1".into(),
//             image_url_1x:
//                 "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/1"
//                     .into(),
//             image_url_2x:
//                 "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/2"
//                     .into(),
//             image_url_4x:
//                 "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/3"
//                     .into(),
//             title: "Turbo".into(),
//             description: "A subscriber of Twitch's monthly premium user service".into(),
//             click_action: Some("turbo".into()),
//             click_url: None,
//         },
//     }];
//     let expected_message = vec![
//         MessageBoby::Emote::<String>(EmoteText::new(
//             "25".into(),
//             "Kappa".into(),
//             "https://static-cdn.jtvnw.net/emoticons/v2/25/default/dark/1.0".into(),
//         )),
//         MessageBoby::Emote::<String>(EmoteText::new(
//             "1902".into(),
//             "Keepo".into(),
//             "https://static-cdn.jtvnw.net/emoticons/v2/1902/default/dark/1.0".into(),
//         )),
//         MessageBoby::Emote::<String>(EmoteText::new(
//             "25".into(),
//             "Kappa".into(),
//             "https://static-cdn.jtvnw.net/emoticons/v2/25/default/dark/1.0".into(),
//         )),
//     ];
//     // let expected_channel = ChannelnMsg::new("#ronni", "Kappa Keepo Kappa");
//
//     let expected_output = OutputMessage {
//         kind: "message".into(),
//         badges: Some(expected_badges),
//         body: expected_message,
//         channel: "#ronni".into(),
//     };
//     assert_eq!(expected, expected_output);
// }
