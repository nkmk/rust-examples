fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[allow(clippy::str_to_string)]
fn main() {
    let s_str = "abc";
    assert_eq!(type_of(&s_str), "&str");

    let s_string = s_str.to_owned();
    assert_eq!(type_of(&s_string), "alloc::string::String");

    let s_string = s_str.to_string();
    assert_eq!(type_of(&s_string), "alloc::string::String");

    let s_string = String::from(s_str);
    assert_eq!(type_of(&s_string), "alloc::string::String");

    // let s_string = s_str.into();
    // error[E0282]: type annotations needed

    let s_string: String = s_str.into();
    assert_eq!(type_of(&s_string), "alloc::string::String");
}
