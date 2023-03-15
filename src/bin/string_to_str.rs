fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    fn test_func(s: &str) -> usize {
        s.len()
    }

    let s_string = String::from("abc");
    assert_eq!(test_func(&s_string), 3);

    let s_str = s_string.as_str();
    assert_eq!(type_of(&s_str), "&str");

    let s_str = &s_string[..];
    assert_eq!(type_of(&s_str), "&str");

    let s_str: &str = &s_string;
    assert_eq!(type_of(&s_str), "&str");

    let s_string_ref = &s_string;
    assert_eq!(type_of(&s_string_ref), "&alloc::string::String");
}
