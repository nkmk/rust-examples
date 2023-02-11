#[allow(unused_variables)]
fn main() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(i8::MIN, -128);

    assert_eq!(u8::MAX, 255);
    assert_eq!(u8::MIN, 0);

    let i: i8 = 100;
    // println!("{}", i + 28);
    // error: this arithmetic operation will overflow

    let s = "100";
    let i: i8 = s.parse().unwrap();

    // debug build
    // println!("{}", i + 28);
    // thread 'main' panicked at 'attempt to add with overflow'

    // release build
    // println!("{}", i + 28);
    // -128

    let i: i8 = 100;
    assert_eq!(i.checked_add(27), Some(127));
    assert_eq!(i.checked_add(28), None);

    let u: u8 = 128;
    assert_eq!(u.wrapping_add(127), 255);
    assert_eq!(u.wrapping_add(128), 0);
    assert_eq!(u.wrapping_add(138), 10);

    assert_eq!(u.wrapping_sub(128), 0);
    assert_eq!(u.wrapping_sub(129), 255);
    assert_eq!(u.wrapping_sub(139), 245);

    let i: i8 = 100;
    assert_eq!(i.wrapping_add(27), 127);
    assert_eq!(i.wrapping_add(28), -128);
    assert_eq!(i.wrapping_add(38), -118);

    let i: i8 = -100;
    assert_eq!(i.wrapping_sub(28), -128);
    assert_eq!(i.wrapping_sub(29), 127);
    assert_eq!(i.wrapping_sub(39), 117);

    let i: i8 = 100;
    assert_eq!(i.overflowing_add(27), (127, false));
    assert_eq!(i.overflowing_add(28), (-128, true));
    assert_eq!(i.overflowing_add(38), (-118, true));

    let i: i8 = -100;
    assert_eq!(i.overflowing_sub(28), (-128, false));
    assert_eq!(i.overflowing_sub(29), (127, true));
    assert_eq!(i.overflowing_sub(39), (117, true));

    let i: i8 = 100;
    assert_eq!(i.saturating_add(27), 127);
    assert_eq!(i.saturating_add(28), 127);
    assert_eq!(i.saturating_add(100), 127);

    let i: i8 = -100;
    assert_eq!(i.saturating_sub(28), -128);
    assert_eq!(i.saturating_sub(29), -128);
    assert_eq!(i.saturating_sub(100), -128);

    use std::num::Wrapping;

    assert_eq!(Wrapping(100_i8) + Wrapping(27), Wrapping(127));
    assert_eq!(Wrapping(100_i8) + Wrapping(28), Wrapping(-128));

    assert_eq!(Wrapping(32_i8) * Wrapping(2), Wrapping(64));
    assert_eq!(Wrapping(32_i8) * Wrapping(4), Wrapping(-128));

    assert_eq!(Wrapping(100_i8).0, 100);
    assert_eq!((Wrapping(100_i8) + Wrapping(28)).0, -128);

    let a: i8 = 64;
    let b: i8 = 32;
    let c: i8 = 2;

    assert_eq!(a.wrapping_add(b.wrapping_mul(c)), -128);
    assert_eq!((Wrapping(a) + Wrapping(b) * Wrapping(c)).0, -128);
}
