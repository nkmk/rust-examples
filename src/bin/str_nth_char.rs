use unicode_segmentation::UnicodeSegmentation;

#[allow(clippy::get_unwrap)]
fn main() {
    let s: &str = "😀👍abc";

    // let c = s[1];
    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    // note: you can use `.chars().nth()` or `.bytes().nth()`

    let c: char = s.chars().nth(1).unwrap();
    assert_eq!(c, '👍');

    assert_eq!(s.chars().nth(100), None);

    let s: &str = "JAPAN🇯🇵";

    assert_eq!(
        s.chars().collect::<Vec<char>>(),
        ['J', 'A', 'P', 'A', 'N', '🇯', '🇵']
    );

    let c: char = s.chars().nth(5).unwrap();
    assert_eq!(c, '🇯');

    let s: &str = "JAPAN🇯🇵";

    assert_eq!(
        s.graphemes(true).collect::<Vec<&str>>(),
        ["J", "A", "P", "A", "N", "🇯🇵"]
    );

    let c_str: &str = s.graphemes(true).nth(5).unwrap();
    assert_eq!(c_str, "🇯🇵");

    let c_str: &str = s.graphemes(true).nth(2).unwrap();
    assert_eq!(c_str, "P");

    let s: &str = "xyz<123>";

    let b: &u8 = s.as_bytes().get(1).unwrap();
    assert_eq!(*b, 121);
    assert_eq!(*b as char, 'y');

    assert_eq!(s.as_bytes().get(100), None);

    let s: &str = "xyz<123>";

    let bytes: &[u8] = s.as_bytes();
    assert_eq!(*bytes.get(2).unwrap() as char, 'z');
    assert_eq!(*bytes.get(5).unwrap() as char, '2');

    let s: &str = "😀👍abc";
    let mut chars = s.chars();

    let pos1 = 1;
    let pos2 = 3;

    assert_eq!(chars.nth(pos1).unwrap(), '👍');
    assert_eq!(chars.nth(pos2 - pos1 - 1).unwrap(), 'b');

    let s: &str = "😀👍abc";
    let v: Vec<char> = s.chars().collect();

    assert_eq!(v[1], '👍');
    assert_eq!(v[3], 'b');

    assert_eq!(*v.get(1).unwrap(), '👍');
    assert_eq!(*v.get(3).unwrap(), 'b');
    assert_eq!(v.get(100), None);
}
