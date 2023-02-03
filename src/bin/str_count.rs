use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s: &str = "abc<123>";
    assert_eq!(s.len(), 8);

    let s: &str = "abc😀👍";
    assert_eq!(s.len(), 11);

    let s: &str = "abc😀👍";
    assert_eq!(s.chars().count(), 5);

    let s: &str = "🇯🇵";
    assert_eq!(s.chars().count(), 2);

    let s: &str = "🇯🇵";
    assert_eq!(s.graphemes(true).count(), 1);

    let s: &str = "🇯🇵JPN😀👍";
    assert_eq!(s.graphemes(true).count(), 6);
}
