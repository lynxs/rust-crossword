#[test]
fn test_trim() {
    let s = String::from("  123  ");
    assert_eq!(s.trim(), "123");
}

#[test]
fn test_trim2() {
    let s = String::from("  123  ").trim().to_string();
    assert_eq!(s, "123");
}

#[test]
fn test_trim3() {
    let s = String::from("  123  ").trim().to_owned();
    assert_eq!(s, "123");
}
