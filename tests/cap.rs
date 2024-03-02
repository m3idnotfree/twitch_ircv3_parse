use ircv3_parse::Ircv3Parse;
use twitch_ircv3_parse::kinds::capabilities::Cap;

#[test]
fn cap_true() {
    let msg = ":tmi.twitch.tv CAP * ACK :twitch.tv/commands twitch.tv/tags";
    let msg = Ircv3Parse::new(msg);
    let expected = Cap::new(msg.params);
    assert_eq!(true, expected.is_capabilities);
}

#[test]
fn cap_false() {
    let msg = "CAP * NAK :twitch.tv/foo";
    let msg = Ircv3Parse::new(msg);
    let expected = Cap::new(msg.params);
    assert_eq!(false, expected.is_capabilities);
}
