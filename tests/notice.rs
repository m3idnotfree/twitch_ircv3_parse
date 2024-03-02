use ircv3_parse::Ircv3Parse;
use twitch_ircv3_parse::kinds::Notice;

#[test]
fn notice_base() {
    let msg = ":tmi.twitch.tv NOTICE * :Improperly formatted auth";
    let msg = Ircv3Parse::new(msg);

    let expected_prefix = ("tmi.twitch.tv", None);
    let expected_command = "NOTICE";
    let expected_channel = "*";
    let expected_message = "Improperly formatted auth";

    let result = Notice::new(msg.tags, msg.prefix, msg.params);
    assert_eq!(result.get_tags(), None);
    assert_eq!(result.get_prefix(), Some(expected_prefix));
    assert_eq!(result.command, expected_command);
    let s = result.get_channel_message();
    assert_eq!(s.message, expected_message);
    assert_eq!(s.channel, expected_channel);
}
