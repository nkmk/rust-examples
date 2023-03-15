#[allow(clippy::uninlined_format_args, clippy::string_add)]
fn main() {
    let s_string = String::from("abc");
    let s_str = "XYZ";

    let s = format!("{}_123_{}", s_string, s_str);
    assert_eq!(s, "abc_123_XYZ");
    assert_eq!(s_string, "abc");
    assert_eq!(s_str, "XYZ");

    let s = format!("{}{}", s_string, s_str);
    assert_eq!(s, "abcXYZ");

    let s = format!("{s_string}{s_str}");
    assert_eq!(s, "abcXYZ");

    let c = '@';
    let s = format!("{s_string}{c}{s_str}");
    assert_eq!(s, "abc@XYZ");

    let mut s_string = String::from("abc");
    let s_str = "XYZ";

    s_string.push_str(s_str);
    assert_eq!(s_string, "abcXYZ");
    assert_eq!(s_str, "XYZ");

    let mut s_string1 = String::from("abc");
    let s_string2 = String::from("XYZ");

    s_string1.push_str(&s_string2);
    assert_eq!(s_string1, "abcXYZ");
    assert_eq!(s_string2, "XYZ");

    let s_string = String::from("abc");
    let s_str = "XYZ";

    let mut s_clone = s_string.clone();
    s_clone.push_str(s_str);
    assert_eq!(s_clone, "abcXYZ");
    assert_eq!(s_string, "abc");
    assert_eq!(s_str, "XYZ");

    let mut s_string = String::from("abc");
    let c = '!';

    s_string.push(c);
    assert_eq!(s_string, "abc!");
    assert_eq!(c, '!');

    let mut s_string = String::from("abc");
    let s_str = "!";

    s_string.push_str(s_str);
    assert_eq!(s_string, "abc!");
    assert_eq!(s_str, "!");

    let mut s_string = String::from("abc");
    let s_str = "XYZ";

    s_string += s_str;
    assert_eq!(s_string, "abcXYZ");
    assert_eq!(s_str, "XYZ");

    let mut s_string1 = String::from("abc");
    let s_string2 = String::from("XYZ");

    s_string1 += &s_string2;
    assert_eq!(s_string1, "abcXYZ");
    assert_eq!(s_string2, "XYZ");

    let s_string = String::from("abc");
    let s_str = "XYZ";

    let s = s_string + s_str;
    assert_eq!(s, "abcXYZ");
    assert_eq!(s_str, "XYZ");

    // assert_eq!(s_string, "abc");
    // error[E0382]: borrow of moved value: `s_string`

    let s_string1 = String::from("abc");
    let s_string2 = String::from("XYZ");

    let s = s_string1 + &s_string2;
    assert_eq!(s, "abcXYZ");
    assert_eq!(s_string2, "XYZ");

    // assert_eq!(s_string1, "abc");
    // error[E0382]: borrow of moved value: `s_string1`

    let s_string = String::from("abc");
    let s_str = "XYZ";

    let mut s = String::new();
    s.push_str(&s_string);
    s.push_str(s_str);
    assert_eq!(s, "abcXYZ");

    let mut s = String::with_capacity(100);
    s.push_str(&s_string);
    s.push_str(s_str);
    assert_eq!(s, "abcXYZ");

    let s_str = "abc";
    let s_string = String::from("XYZ");

    let mut s_str_string = String::from(s_str);
    s_str_string.push_str(&s_string);
    assert_eq!(s_str_string, "abcXYZ");

    let s = String::from(s_str) + &s_string;
    assert_eq!(s, "abcXYZ");

    let s_str = "abc";
    let s_string = String::from("XYZ");

    let mut s = String::new();
    s.push_str(s_str);
    s.push_str(&s_string);
    assert_eq!(s, "abcXYZ");

    let s = format!("{}{}", s_str, s_string);
    assert_eq!(s, "abcXYZ");
}
