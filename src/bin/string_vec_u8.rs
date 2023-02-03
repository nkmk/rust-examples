fn main() {
    let s: String = String::from("abc😀");
    let v: Vec<u8> = s.into_bytes();
    assert_eq!(v, [97, 98, 99, 240, 159, 152, 128]);

    let s: String = String::from("abc😀");
    let b: &[u8] = s.as_bytes();
    assert_eq!(b, [97, 98, 99, 240, 159, 152, 128]);

    let v: Vec<u8> = vec![97, 98, 99, 240, 159, 152, 128];
    let s: String = String::from_utf8(v).unwrap();
    assert_eq!(s, "abc😀");

    let v: Vec<u8> = vec![97, 98, 99, 255];
    let is_err: bool = String::from_utf8(v).is_err();
    assert!(is_err);

    let v: Vec<u8> = vec![97, 98, 99, 240, 159, 152, 128];
    let s: String = unsafe { String::from_utf8_unchecked(v) };
    assert_eq!(s, "abc😀");

    let v: Vec<u8> = vec![97, 98, 99, 240, 159, 152, 128];
    let s: String = String::from_utf8_lossy(&v).into_owned();
    assert_eq!(s, "abc😀");

    let v: Vec<u8> = vec![97, 98, 99, 255];
    let s: String = String::from_utf8_lossy(&v).into_owned();
    assert_eq!(s, "abc�");

    let b: [u8; 4] = [97, 98, 99, 255];
    let s: String = String::from_utf8_lossy(&b).into_owned();
    assert_eq!(s, "abc�");

    let mut s = String::from("xyz_");
    let b: [u8; 4] = [97, 98, 99, 255];
    let cow = String::from_utf8_lossy(&b);

    s.push_str(&cow);

    assert_eq!(s, "xyz_abc�");
}
