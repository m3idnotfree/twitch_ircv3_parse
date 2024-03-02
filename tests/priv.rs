use ircv3_parse::Ircv3Parse;
use pretty_assertions::assert_eq;
#[cfg(test)]
use std::collections::HashMap;
use twitch_ircv3_parse::{
    kinds::PrivMsg,
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
            ("tmi-sent-ts".into(), "1507246572675"),
            ("user-id", "1337"),
            ("user-type", "global_mod"),
            ("id", "b34ccfc7-4977-403a-8a94-33c6bac34fb8"),
            ("turbo", "1"),
            ("badge-info", ""),
            ("display-name", "ronni"),
            ("color", "#0D4200"),
            ("mod", "0"),
            ("subscriber", "0"),
            ("room-id", "1337"),
        ("badges","turbo/1"),
        ("emotes","25:0-4,12-16/1902:6-10"),

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
            data: "25:0-4,12-16/1902:6-10",
        }),
    );
    // assert_eq!(result.badges, Some(Badges { data: "turbo/1" },));
    assert_eq!(result.get_badges(), Some(BadgesTag { data: "turbo/1" },));
}

// #[test]
// fn priv_postform() {
//     let msg = "@badge-info=;badges=turbo/1;color=#0D4200;display-name=ronni;emotes=25:0-4,12-16/1902:6-10;id=b34ccfc7-4977-403a-8a94-33c6bac34fb8;mod=0;room-id=1337;subscriber=0;tmi-sent-ts=1507246572675;turbo=1;user-id=1337;user-type=global_mod :ronni!ronni@ronni.tmi.twitch.tv PRIVMSG #ronni :Kappa Keepo Kappa\r\n";
//     let msg = Ircv3Parse::new(msg);
//     let expected = PrivMsg::new(msg.tags, msg.prefix, msg.params);
// }
