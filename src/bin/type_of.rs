fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    assert_eq!(std::any::type_name::<i32>(), "i32");
    assert_eq!(std::any::type_name::<bool>(), "bool");

    assert_eq!(std::any::type_name::<&str>(), "&str");
    assert_eq!(std::any::type_name::<&'static str>(), "&str");

    assert_eq!(std::any::type_name::<String>(), "alloc::string::String");
    assert_eq!(std::any::type_name::<Vec<i32>>(), "alloc::vec::Vec<i32>");
    assert_eq!(
        std::any::type_name::<Option<i32>>(),
        "core::option::Option<i32>"
    );

    assert_eq!(
        std::any::type_name::<Vec<String>>(),
        "alloc::vec::Vec<alloc::string::String>"
    );
    assert_eq!(
        std::any::type_name::<Option<String>>(),
        "core::option::Option<alloc::string::String>"
    );

    struct MyStruct(i32);
    assert_eq!(std::any::type_name::<MyStruct>(), "type_of::main::MyStruct");

    type MyAlias = u8;
    assert_eq!(std::any::type_name::<MyAlias>(), "u8");

    let i = 100;
    assert_eq!(type_of(&i), "i32");

    let u = 100_u8;
    assert_eq!(type_of(&u), "u8");

    let b = true;
    assert_eq!(type_of(&b), "bool");

    let s = "abc";
    assert_eq!(type_of(&s), "&str");

    let t = (100, 100);
    assert_eq!(type_of(&t), "(i32, i32)");

    let v = vec![String::from("abc")];
    assert_eq!(type_of(&v), "alloc::vec::Vec<alloc::string::String>");

    let i = 100;
    let ref_i = &i;
    assert_eq!(type_of(&ref_i), "&i32");

    let mut i = 100;
    let mut_ref_i = &mut i;
    assert_eq!(type_of(&mut_ref_i), "&mut i32");

    let a = [0, 1, 2];
    assert_eq!(type_of(&a), "[i32; 3]");

    let s = &a[1..];
    assert_eq!(type_of(&s), "&[i32]");

    let my_struct = MyStruct(100);
    assert_eq!(type_of(&my_struct), "type_of::main::MyStruct");

    let u: MyAlias = 100;
    assert_eq!(type_of(&u), "u8");

    let it = [0, 1, 2].iter().enumerate();
    assert_eq!(
        type_of(&it),
        "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<i32>>"
    );

    let result = "100".parse::<i32>();
    assert_eq!(
        type_of(&result),
        "core::result::Result<i32, core::num::error::ParseIntError>"
    );
}
