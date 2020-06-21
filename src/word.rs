fn is_same_chars(set: &str, word: &str) -> bool {
    let mut set_chars: Vec<char> = set.chars().collect();
    for c in word.chars() {
        let _ = match set_chars.iter().position(|&x| x == c) {
            Some(x) => set_chars.remove(x),
            None => return false
        };
    }
    true
}

#[test]
fn test_trim() {
    let s = String::from("  123  ");
    assert_eq!(s.trim(), "123");
    let s = String::from("  123  ").trim().to_string();
    assert_eq!(s, "123");
    let s = String::from("  123  ").trim().to_owned();
    assert_eq!(s, "123");
}

#[test]
fn test_same_chars() {
    assert_eq!(is_same_chars("клоун", "лук"), true);
    assert_eq!(is_same_chars("клоун", "лув"), false);
    assert_eq!(is_same_chars("арарат", "ара"), true);
}