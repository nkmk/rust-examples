#[allow(unused_variables)]
fn main() {
    let i = 5;

    assert!(i < 10);

    // assert!(i > 10);
    // thread 'main' panicked at 'assertion failed: i > 10'

    // assert!(i > 10, "CUSTOM MESSAGE: i = {}", i);
    // thread 'main' panicked at 'CUSTOM MESSAGE: i = 5'

    let a = 2;
    let b = 4;

    assert_eq!(a * 2, b);

    // assert_eq!(a * 3, b);
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //  left: `6`,
    // right: `4`'

    let i_32: i32 = 100;
    let i_64: i64 = 100;

    // assert_eq!(i_32, i_64);
    // error[E0308]: mismatched types

    let s_str: &str = "abc";
    let s_string: String = String::from("abc");

    assert_eq!(s_str, s_string);

    let a = 2;
    let b = 4;

    assert_ne!(a * 3, b);

    // assert_ne!(a * 2, b);
    // thread 'main' panicked at 'assertion failed: `(left != right)`
    //  left: `4`,
    // right: `4`'
}
