#[cfg(test)]
use pretty_assertions::assert_eq;
use twitch_ircv3_parse::tags::EmotesTag;

#[test]
fn emotes_base() {
    let msg = "25:0-4,12-16/1902:6-10";
    let result = EmotesTag::parse(msg);
    let expected = vec![("25", 0, 4), ("1902", 6, 10), ("25", 12, 16)];

    assert_eq!(result, Some(expected))
}

#[test]
fn emotes_one() {
    let msg = "25:0-4,12-16";
    let result = EmotesTag::parse(msg);
    let expected = vec![
        ("25", 0, 4),
        // ("1902".to_string(), 6, 10),
        ("25", 12, 16),
    ];

    assert_eq!(result, Some(expected))
}

#[test]
fn emotes_base_very_long() {
    let msg = "301544926:14-21/30259:26-32/81274:49-54/305954156:56-63,65-72/25:5-9,39-43,77-81";
    let result = EmotesTag::parse(msg);
    let expected = vec![
        ("25", 5, 9),
        ("301544926", 14, 21),
        ("30259", 26, 32),
        ("25", 39, 43),
        ("81274", 49, 54),
        ("305954156", 56, 63),
        ("305954156", 65, 72),
        ("25", 77, 81),
    ];

    assert_eq!(result, Some(expected))
}

#[test]
fn emotes_empty() {
    let msg = "";
    let result = EmotesTag::parse(msg);

    assert_eq!(result, None)
}

// #[test]
// fn emotes_with_message() {
//     let emotes = "301544926:14-21/30259:26-32/81274:49-54/305954156:56-63,65-72/25:5-9,39-43,77-81";
//     let msg = "fffs Kappa  f SirPrise He HeyGuys eefq Kappa ssq VoHiYo PogChamp PogChamp ff Kappa fffes ff ssf peucew euufefny ewu";
//
//     let emotes_result = Emotes::parse(emotes).unwrap();
//     let (_, result) = Emotes::get_data(msg, emotes_result).unwrap();
//
//     assert_eq!(result, vec!["".to_string()])
// }
