use twitch_ircv3_parse::TwitchMessage;

#[test]
fn notice_base() {
    let msg = ":tmi.twitch.tv NOTICE * :Improperly formatted auth";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "NOTICE".to_string());
    assert_eq!(msg.params.channel(), Some("*"));
    assert_eq!(msg.params.message(), Some("Improperly formatted auth"));
    assert_eq!(msg.prefix.server_nick(), Some("tmi.twitch.tv"));
    assert_eq!(msg.prefix.user(), None);
}
