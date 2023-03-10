use unicode_segmentation::UnicodeSegmentation;

#[allow(clippy::get_unwrap)]
fn main() {
    let s: &str = "abcde";
    let s_sub: &str = &s[1..4];
    assert_eq!(s_sub, "bcd");

    let s: String = String::from("abcde");
    let s_sub: &str = &s[1..4];
    assert_eq!(s_sub, "bcd");

    let s_sub: String = String::from(&s[1..4]);
    assert_eq!(s_sub, "bcd");

    let s: &str = "abcde";
    // let s_sub: &str = &s[1..100];
    // thread 'main' panicked at 'byte index 100 is out of bounds of `abcde`'

    assert_eq!(s.get(1..4).unwrap(), "bcd");
    assert_eq!(s.get(1..100), None);

    let s: &str = "šššÆšš";
    // let s_sub: &str = &s[1..4];
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'š' (bytes 0..4) of `šššÆšš`

    assert_eq!(
        s.char_indices().collect::<Vec<(usize, char)>>(),
        [(0, 'š'), (4, 'š'), (8, 'šÆ'), (12, 'š'), (16, 'š')]
    );

    let byte_position = s.char_indices().nth(3).unwrap().0;
    assert_eq!(byte_position, 12);

    let byte_start = s.char_indices().nth(1).unwrap().0;
    let byte_end = s.char_indices().nth(4).unwrap().0;
    let s_sub: &str = &s[byte_start..byte_end];
    assert_eq!(s_sub, "ššÆš");

    let s: &str = "šššÆšš";
    let s_sub: &str = substring(s, 1, 3);
    assert_eq!(s_sub, "ššÆš");

    let s: String = String::from("šššÆšš");
    let s_sub: &str = substring(&s, 1, 3);
    assert_eq!(s_sub, "ššÆš");

    let s: &str = "šššÆšš";
    assert_eq!(substring(s, 1, 0), "");
    assert_eq!(substring(s, 100, 3), "");
    assert_eq!(substring(s, 1, 100), "ššÆšš");

    let s: &str = "šššÆšš";
    assert_eq!(substring_option(s, 1, 0).unwrap(), "");
    assert_eq!(substring_option(s, 100, 3), None);
    assert_eq!(substring_option(s, 1, 100), None);

    let s: &str = "šÆšµJPš";
    assert_eq!(
        s.char_indices().collect::<Vec<(usize, char)>>(),
        [(0, 'šÆ'), (4, 'šµ'), (8, 'J'), (9, 'P'), (10, 'š')]
    );

    let s: &str = "šÆšµJPš";
    assert_eq!(
        s.grapheme_indices(true).collect::<Vec<(usize, &str)>>(),
        [(0, "šÆšµ"), (8, "J"), (9, "P"), (10, "š")]
    );

    let s: &str = "šÆšµJPš";
    assert_eq!(substring_grapheme(s, 0, 2, true), "šÆšµJ");

    let s: String = String::from("šÆšµJPš");
    assert_eq!(substring_grapheme(&s, 0, 2, true), "šÆšµJ");

    let s: &str = "abcdešššÆšš";
    let mut ci = s.char_indices();

    let pos1 = 1;
    let len1 = 2;
    let pos2 = 5;
    let len2 = 3;

    let byte_start1 = ci.nth(pos1).unwrap().0;
    let byte_end1 = ci.nth(len1 - 1).unwrap().0;
    let byte_start2 = ci.nth(pos2 - pos1 - len1 - 1).unwrap().0;
    let byte_end2 = ci.nth(len2 - 1).unwrap().0;

    let s_sub1: &str = &s[byte_start1..byte_end1];
    let s_sub2: &str = &s[byte_start2..byte_end2];

    assert_eq!(s_sub1, "bc");
    assert_eq!(s_sub2, "šššÆ");

    let s: &str = "abcdešššÆšš";
    let v_char: Vec<char> = s.chars().collect();

    let s_sub1: String = v_char[1..3].iter().collect();
    let s_sub2: String = v_char[5..8].iter().collect();
    let s_sub3: String = v_char[2..6].iter().collect();

    assert_eq!(s_sub1, "bc");
    assert_eq!(s_sub2, "šššÆ");
    assert_eq!(s_sub3, "cdeš");

    let s: &str = "abcdešÆšµššÆšš";
    let v_str: Vec<&str> = s.graphemes(true).collect();

    let s_sub1: String = v_str[1..3].concat();
    let s_sub2: String = v_str[5..8].concat();
    let s_sub3: String = v_str[2..6].concat();

    assert_eq!(s_sub1, "bc");
    assert_eq!(s_sub2, "šÆšµššÆ");
    assert_eq!(s_sub3, "cdešÆšµ");

    assert_eq!(v_char.get(1..3).unwrap().iter().collect::<String>(), "bc");
    assert_eq!(v_char.get(1..100), None);
    assert_eq!(v_str.get(1..3).unwrap().concat(), "bc");
    assert_eq!(v_str.get(1..100), None);
}

fn substring(s: &str, start: usize, length: usize) -> &str {
    if length == 0 {
        return "";
    }

    let mut ci = s.char_indices();
    let byte_start = match ci.nth(start) {
        Some(x) => x.0,
        None => return "",
    };

    match ci.nth(length - 1) {
        Some(x) => &s[byte_start..x.0],
        None => &s[byte_start..],
    }
}

fn substring_option(s: &str, start: usize, length: usize) -> Option<&str> {
    if length == 0 {
        return Some("");
    }

    let mut ci = s.char_indices();
    let byte_start = match ci.nth(start) {
        Some(x) => x.0,
        None => return None,
    };

    match ci.nth(length - 1) {
        Some(x) => Some(&s[byte_start..x.0]),
        None => None,
    }
}

fn substring_grapheme(s: &str, start: usize, length: usize, is_extended: bool) -> &str {
    if length == 0 {
        return "";
    }

    let mut gi = s.grapheme_indices(is_extended);
    let byte_start = match gi.nth(start) {
        Some(x) => x.0,
        None => return "",
    };

    match gi.nth(length - 1) {
        Some(x) => &s[byte_start..x.0],
        None => &s[byte_start..],
    }
}
