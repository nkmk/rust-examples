fn main() {
    let s: &str = "abc😀";
    let b: &[u8] = s.as_bytes();
    assert_eq!(b, [97, 98, 99, 240, 159, 152, 128]);

    let b: [u8; 7] = [97, 98, 99, 240, 159, 152, 128];
    let s: &str = std::str::from_utf8(&b).unwrap();
    assert_eq!(s, "abc😀");

    let b: [u8; 4] = [97, 98, 99, 255];
    let is_err: bool = std::str::from_utf8(&b).is_err();
    assert!(is_err);

    let b: [u8; 7] = [97, 98, 99, 240, 159, 152, 128];
    let s: &str = unsafe { std::str::from_utf8_unchecked(&b) };
    assert_eq!(s, "abc😀");

    let v: Vec<u8> = vec![97, 98, 99, 240, 159, 152, 128];
    let s: &str = std::str::from_utf8(&v).unwrap();
    assert_eq!(s, "abc😀");

    let b: &'static [u8; 7] = b"abc\xF0\x9F\x98\x80";
    let s: &str = std::str::from_utf8(b).unwrap();
    assert_eq!(s, "abc😀");
}
