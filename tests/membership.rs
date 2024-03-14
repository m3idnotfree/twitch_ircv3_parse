use twitch_ircv3_parse::TwitchMessage;

#[test]
fn join() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "JOIN");
    assert_eq!(msg.params.channel(), Some("#<channel>"));
    assert_eq!(msg.params.message(), None,);
}

#[test]
fn join_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>\r\n";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "JOIN");
    assert_eq!(msg.params.channel(), Some("#<channel>"));
    assert_eq!(msg.params.message(), None,);
}

#[test]
fn part() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv PART #<channel>";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "PART");
    assert_eq!(msg.params.channel(), Some("#<channel>"));
    assert_eq!(msg.params.message(), None,);
}

#[test]
fn part_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv PART #<channel>\r\n";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "PART");
    assert_eq!(msg.params.channel(), Some("#<channel>"));
    assert_eq!(msg.params.message(), None,);
}
