use ircv3_parse::Ircv3Parse;
use twitch_ircv3_parse::kinds::Member;

#[test]
fn join() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>";
    let result = Ircv3Parse::new(msg);
    let result = Member::new("JOIN".into(), result.prefix, result.params);
    assert_eq!(result.channel, "#<channel>");
    assert_eq!(result.command, "JOIN");
    assert_eq!(
        result.user,
        Some(("<user>".into(), Some("<user>@<user>.tmi.twitch.tv".into())))
    );
}

#[test]
fn join_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv JOIN #<channel>\r\n";
    let result = Ircv3Parse::new(msg);
    let result = Member::new("JOIN".into(), result.prefix, result.params);
    assert_eq!(result.channel, "#<channel>");
    assert_eq!(result.command, "JOIN");
    assert_eq!(
        result.user,
        Some(("<user>".into(), Some("<user>@<user>.tmi.twitch.tv".into())))
    );
}

#[test]
fn part() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv PART #<channel>";
    let result = Ircv3Parse::new(msg);
    let result = Member::new("PART".into(), result.prefix, result.params);
    assert_eq!(result.channel, "#<channel>");
    assert_eq!(result.command, "PART");
    assert_eq!(
        result.user,
        Some(("<user>".into(), Some("<user>@<user>.tmi.twitch.tv".into())))
    );
}

#[test]
fn part_rn() {
    let msg = ":<user>!<user>@<user>.tmi.twitch.tv PART #<channel>\r\n";
    let result = Ircv3Parse::new(msg);
    let result = Member::new("PART".into(), result.prefix, result.params);
    assert_eq!(result.channel, "#<channel>");
    assert_eq!(result.command, "PART");
    assert_eq!(
        result.user,
        Some(("<user>".into(), Some("<user>@<user>.tmi.twitch.tv".into())))
    );
}
