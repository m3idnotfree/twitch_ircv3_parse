use twitch_ircv3_parse::{utils::is_capabilities, TwitchMessage};

#[test]
fn cap_true() {
    let msg = ":tmi.twitch.tv CAP * ACK :twitch.tv/commands twitch.tv/tags";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "CAP");
    assert!(is_capabilities(msg.params));
}

#[test]
fn cap_false() {
    let msg = "CAP * NAK :twitch.tv/foo";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "CAP");
    assert!(!is_capabilities(msg.params));
}
