#[cfg(test)]
use pretty_assertions::assert_eq;
use twitch_ircv3_parse::tags::BadgesTag;
#[test]
fn badges_base() {
    let msg = "turbo/1";
    let result = BadgesTag::new(msg);
    assert_eq!(result.parse(), Some(vec![("turbo", "1")]))
}
#[test]
fn badges_non() {
    let msg = "";
    let result = BadgesTag::new(msg);
    assert_eq!(result.parse(), None)
}

#[test]
fn badges_two() {
    let msg = "vip/1,partner/1";
    let result = BadgesTag::new(msg);
    assert_eq!(result.parse(), Some(vec![("vip", "1"), ("partner", "1")]))
}
