#[allow(clippy::zero_divided_by_zero, clippy::eq_op, unused_variables)]
fn main() {
    let f = 10.0 / 0.0;
    assert_eq!(f, f64::INFINITY);

    let f = -10.0 / 0.0;
    assert_eq!(f, f64::NEG_INFINITY);

    let f: f64 = 0.0 / 0.0;
    assert!(f.is_nan());

    // let i = 10 / 0;
    // error: this operation will panic at runtime

    let s = "0";
    let divisor: i32 = s.parse().unwrap();

    // let i = 10 / divisor;
    // thread 'main' panicked at 'attempt to divide by zero'

    let dividend: i32 = 10;

    assert_eq!(dividend.checked_div(0), None);
    assert_eq!(dividend.checked_div(2), Some(5));
    assert_eq!(dividend.checked_div(3), Some(3));

    assert_eq!(i8::MIN, -128);
    assert_eq!(i8::MAX, 127);

    let i_min = i8::MIN;

    assert_eq!(i_min.checked_div(-1), None);
}
