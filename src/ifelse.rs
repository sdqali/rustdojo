pub fn compare(x: u32, y: u32) -> i8 {
    if x < y {
        -1
    } else if x > y {
        1
    } else {
        0
    }
}

#[test]
fn test_finds_bigger() {
    assert_eq!(1, compare(12, 0));
}

#[test]
fn test_finds_smaller() {
    assert_eq!(-1, compare(76, 100));
}

#[test]
fn test_finds_equal() {
    assert_eq!(0, compare(54, 54));
}
