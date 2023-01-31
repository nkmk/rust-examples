#[allow(
    unused_variables,
    clippy::inconsistent_digit_grouping,
    clippy::just_underscores_and_digits
)]
fn main() {
    let i = 100_000_000;
    let i = 0b1000_0000;
    let f = 0.123_456_789;

    let i = 100_i32;
    let f = 0.123_f64;

    let i = 10___0_______0___;
    assert_eq!(i, 1000);

    let f = 0_.123;
    assert_eq!(f, 0.123);

    // let f = 0._123;
    // error[E0610]: `{integer}` is a primitive type and therefore doesn't have fields

    // let i = __100;
    // error[E0425]: cannot find value `__100` in this scope

    let __100 = "abc";
    assert_eq!(__100, "abc");
}
