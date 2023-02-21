fn main() {
    let s = String::from("abc");
    assert_eq!(s.len(), 3);
    assert_eq!(s.capacity(), 3);

    let s = String::from("😀");
    assert_eq!(s.len(), 4);
    assert_eq!(s.capacity(), 4);

    let mut s = String::with_capacity(10);
    s.push_str("abc");
    assert_eq!(s.len(), 3);
    assert_eq!(s.capacity(), 10);

    let mut s = String::from("abc");
    assert_eq!(s.len(), 3);
    assert_eq!(s.capacity(), 3);

    s.reserve(10);
    assert_eq!(s.capacity(), 13);

    s.shrink_to(5);
    assert_eq!(s.capacity(), 5);

    s.shrink_to_fit();
    assert_eq!(s.capacity(), 3);
}
