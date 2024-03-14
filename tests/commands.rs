use twitch_ircv3_parse::{utils::parse_hosttarget, TwitchMessage};

#[test]
fn hosttarget() {
    let msg = ":tmi.twitch.tv HOSTTARGET #<hosting-channel> :[-|<channel>] 1";
    let msg = TwitchMessage::parse(msg);

    assert_eq!(msg.command.to_string(), "HOSTTARGET");
    let msg = parse_hosttarget(msg.params);

    assert_eq!(msg.hosting_channel, "#<hosting-channel>".to_string());
    assert_eq!(msg.channel, "[-|<channel>]".to_string());
    assert_eq!(msg.number_of_viewers, 1);
}
