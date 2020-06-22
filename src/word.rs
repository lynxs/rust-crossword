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

pub fn is_same_chars_bits(set: &str, word: &str) -> bool {
    if word.len() > set.len() { return false; }
    
    // split word to 32-bit chunks with u32 mask for each if word with len > 32
    if word.len() > 32 { panic!("max word length 32 chars"); }

    let mut mask = 0_u32;
    
    for c in word.chars() {
        let pos = set.chars().enumerate().position(|(i,x)| x == c && (mask & 1<<i) == 0);
        match pos {
            Some(x) => mask |= 1<<x,
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

#[test]
fn test_same_chars_bits() {
    assert_eq!(is_same_chars_bits("клоун", "лук"), true);
    assert_eq!(is_same_chars_bits("клоун", "лув"), false);
    assert_eq!(is_same_chars_bits("арарат", "ара"), true);
    assert_eq!(is_same_chars_bits("арарат", "арарат"), true);
    assert_eq!(is_same_chars_bits("арарат", "арарата"), false);
}