#[allow(clippy::eq_op, clippy::cast_lossless)]
fn main() {
    let result: bool = 2 + 2 == 4;
    println!("{result}");
    // true

    let result: bool = 2 + 2 == 5;
    println!("{result}");
    // false

    let result: bool = 2 + 2 != 4;
    println!("{result}");
    // false

    let result: bool = 2 + 2 != 5;
    println!("{result}");
    // true

    let a: i32 = 10;
    let b: i32 = 10;
    let c: i32 = 100;

    assert!(a == b);
    assert!(a.eq(&b));

    assert!(a != c);
    assert!(a.ne(&c));

    let i_32: i32 = 100;
    let i_64: i64 = 100;

    // assert!(i_32 == i_64);
    // assert!(i_32.eq(&i_64));
    // error[E0308]: mismatched types

    assert!(i_32 as i64 == i_64);

    let s_str: &str = "abc";
    let s_string: String = String::from("abc");

    assert!(s_str == s_string);
    assert!(s_str.eq(&s_string));

    assert!(s_string == s_str);
    assert!(s_string.eq(&s_str));

    let v: Vec<i32> = vec![10, 20, 30];
    let a: [i32; 3] = [10, 20, 30];

    assert!(v == a);
    assert!(v.eq(&a));

    // assert!(a == v);
    // assert!(a.eq(&v));
    // error[E0277]: can't compare `[i32; 3]` with `std::vec::Vec<i32>`
}
