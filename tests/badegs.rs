use twitch_ircv3_parse::tags::parse_badges;

#[test]
fn badges_empty() {
    let msg = "";
    let result = parse_badges(msg);

    assert_eq!(result.as_ref(), &None)
}

#[test]
fn badges_base() {
    let msg = "turbo/1";
    let result = parse_badges(msg);

    assert_eq!(result.as_ref(), &Some(vec![("turbo".into(), "1".into())]))
}

#[test]
fn badges_two() {
    let msg = "vip/1,partner/1";
    let result = parse_badges(msg);

    assert_eq!(
        result.as_ref(),
        &Some(vec![
            ("vip".into(), "1".into()),
            ("partner".into(), "1".into())
        ])
    )
}
