use ircv3_parse::Ircv3Parse;
use twitch_ircv3_parse::kinds::capabilities::commands::HostTarget;

#[test]
fn hosttarget() {
    let msg = ":tmi.twitch.tv HOSTTARGET #<hosting-channel> :[-|<channel>] <number-of-viewers>";
    let result = Ircv3Parse::new(msg);
    let result = HostTarget::new(result.params);
    assert_eq!(result.command, "HOSTTARGET");
    assert_eq!(result.host_channel, "#<hosting-channel>");
    assert_eq!(result.channel, "[-|<channel>]");
    assert_eq!(result.number_of_viewers, "<number-of-viewers>");
}
