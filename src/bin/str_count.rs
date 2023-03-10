use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s: &str = "abc<123>";
    assert_eq!(s.len(), 8);

    let s: &str = "abcðð";
    assert_eq!(s.len(), 11);

    let s: &str = "abcðð";
    assert_eq!(s.chars().count(), 5);

    let s: &str = "ð¯ðµ";
    assert_eq!(s.chars().count(), 2);

    let s: &str = "ð¯ðµ";
    assert_eq!(s.graphemes(true).count(), 1);

    let s: &str = "ð¯ðµJPNðð";
    assert_eq!(s.graphemes(true).count(), 6);
}
