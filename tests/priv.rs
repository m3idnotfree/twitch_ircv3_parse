use ircv3_parse::Ircv3Parse;
use pretty_assertions::assert_eq;
#[cfg(test)]
use std::collections::HashMap;
use std::fs;
use twitch_highway::badges::{Badge, BadgeResponse, Versions};
use twitch_ircv3_parse::{
    kinds::{EmoteText, MessageBoby, OutputMessage, PrivMsg},
    tags::{BadgesTag, EmotesTag},
};

#[test]
fn priv_base_unit() {
    let msg = ":foo!foo@foo.tmi.twitch.tv PRIVMSG #bar :bleedPurple\r\n";
    let msg = Ircv3Parse::new(msg);

    let expected_prefix = ("foo", Some("foo@foo.tmi.twitch.tv"));
    let expected_command = "PRIVMSG";
    let expected_channel = "#bar";
    let expected_message = "bleedPurple";
    // let expected = PrivMsg::new(msg.tags.hashmap_str(), msg.prefix.to_str(), msg.message);
    let result = PrivMsg::new(msg.tags, msg.prefix, msg.params);
    assert_eq!(result.get_tags(), None);
    assert_eq!(result.get_prefix(), Some(expected_prefix));
    assert_eq!(result.command, expected_command);
    let c_m = result.get_channel_message();

    assert_eq!(c_m.message, expected_message);
    assert_eq!(c_m.channel, expected_channel);
}

#[test]
fn privmsg_chat_room_unit() {
    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n";
    let msg = Ircv3Parse::new(msg);
    let result = PrivMsg::new(msg.tags, msg.prefix, msg.params);
    let expected_tags =
    // Some((
        HashMap::from([
            ("tmi-sent-ts".into(), "1507246572675".into()),
            ("user-id".into(), "1337".into()),
            ("user-type".into(), "global_mod".into()),
            ("id".into() ,"b34ccfc7-4977-403a-8a94-33c6bac34fb8".into()),
            ("turbo".into() ,"1".into()),
            ("badge-info".into(), "".into()),
            ("display-name".into(), "ronni".into()),
            ("color".into() ,"#0D4200".into()),
            ("mod".into(), "0".into()),
            ("subscriber".into(), "0".into()),
            ("room-id".into(),"1337".into()),
        ("badges".into(),"turbo/1".into()),
        ("emotes".into(),"25:0-4,12-16/1902:6-10".into()),

    ]);

    let expected_prefix = ("ronni", Some("ronni@ronni.tmi.twitch.tv"));
    let expected_command = "PRIVMSG";
    let expected_channel = "#ronni";
    let expected_message = "Kappa Keepo Kappa";

    assert_eq!(result.get_tags(), Some(expected_tags));
    assert_eq!(result.get_prefix(), Some(expected_prefix));
    assert_eq!(result.command, expected_command);
    let c_m = result.get_channel_message();

    assert_eq!(c_m.message, expected_message);
    assert_eq!(c_m.channel, expected_channel);
    assert_eq!(
        // result.emotes,
        result.get_emotes(),
        Some(EmotesTag {
            data: "25:0-4,12-16/1902:6-10".into()
        }),
    );
    // assert_eq!(result.badges, Some(Badges { data: "turbo/1" },));
    assert_eq!(
        result.get_badges(),
        Some(BadgesTag {
            data: "turbo/1".into()
        },)
    );
}

#[test]
fn priv_postform() {
    let badges_json = fs::read_to_string("badges.json").unwrap();
    let badges_json: BadgeResponse = serde_json::from_str(&badges_json).unwrap();

    let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n";
    let msg = Ircv3Parse::new(msg);
    let expected = PrivMsg::new(msg.tags, msg.prefix, msg.params);
    let expected = expected.output_json(&badges_json);

    let expected_badges = vec![Badge {
        set_id: "turbo".into(),
        versions: Versions {
            id: "1".into(),
            image_url_1x:
                "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/1"
                    .into(),
            image_url_2x:
                "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/2"
                    .into(),
            image_url_4x:
                "https://static-cdn.jtvnw.net/badges/v1/bd444ec6-8f34-4bf9-91f4-af1e3428d80f/3"
                    .into(),
            title: "Turbo".into(),
            description: "A subscriber of Twitch's monthly premium user service".into(),
            click_action: Some("turbo".into()),
            click_url: None,
        },
    }];
    let expected_message = vec![
        MessageBoby::Emote::<String>(EmoteText::new(
            "25".into(),
            "Kappa".into(),
            "https://static-cdn.jtvnw.net/emoticons/v2/25/default/dark/1.0".into(),
        )),
        MessageBoby::Emote::<String>(EmoteText::new(
            "1902".into(),
            "Keepo".into(),
            "https://static-cdn.jtvnw.net/emoticons/v2/1902/default/dark/1.0".into(),
        )),
        MessageBoby::Emote::<String>(EmoteText::new(
            "25".into(),
            "Kappa".into(),
            "https://static-cdn.jtvnw.net/emoticons/v2/25/default/dark/1.0".into(),
        )),
    ];
    // let expected_channel = ChannelnMsg::new("#ronni", "Kappa Keepo Kappa");

    let expected_output = OutputMessage {
        kind: "message".into(),
        badges: Some(expected_badges),
        body: expected_message,
        channel: "#ronni".into(),
    };
    assert_eq!(expected, expected_output);
}
