#[allow(clippy::bool_assert_comparison)]
fn main() {
    let s: &str = "100";
    let i = s.parse::<i32>().unwrap();
    assert_eq!(i, 100);

    let i: i32 = s.parse().unwrap();
    assert_eq!(i, 100);

    let s: &str = "abc";
    let is_err = s.parse::<i32>().is_err();
    assert!(is_err);

    let s: String = String::from("100");
    let i = s.parse::<i32>().unwrap();
    assert_eq!(i, 100);

    let s: &str = "false";
    let b = s.parse::<bool>().unwrap();
    assert_eq!(b, false);

    assert_eq!("1.23".parse::<f64>().unwrap(), 1.23);
    assert_eq!("123".parse::<f64>().unwrap(), 123.0);
    assert_eq!("123.".parse::<f64>().unwrap(), 123.0);
    assert_eq!(".123".parse::<f64>().unwrap(), 0.123);
    assert_eq!("1.23e2".parse::<f64>().unwrap(), 123.0);
    assert_eq!("1.23e-2".parse::<f64>().unwrap(), 0.0123);

    assert_eq!("inf".parse::<f64>().unwrap(), f64::INFINITY);
    assert_eq!("-inf".parse::<f64>().unwrap(), f64::NEG_INFINITY);
    assert_eq!("infinity".parse::<f64>().unwrap(), f64::INFINITY);
    assert!("nan".parse::<f64>().unwrap().is_nan());

    let s: &str = "100";
    assert_eq!(i32::from_str_radix(s, 2).unwrap(), 4);
    assert_eq!(i32::from_str_radix(s, 8).unwrap(), 64);
    assert_eq!(i32::from_str_radix(s, 16).unwrap(), 256);

    let s: String = String::from("ff");
    assert_eq!(i32::from_str_radix(&s, 16).unwrap(), 255);

    let s: &str = "0xFF";
    assert_eq!(i32::from_str_radix(&s[2..], 16).unwrap(), 255);

    assert_eq!(parse_with_prefix("0b100").unwrap(), 4);
    assert_eq!(parse_with_prefix("0o100").unwrap(), 64);
    assert_eq!(parse_with_prefix("0x100").unwrap(), 256);

    assert!(parse_with_prefix("0x").is_err());
    assert!(parse_with_prefix("0a100").is_err());
    assert!(parse_with_prefix("0xZZZ").is_err());

    assert_eq!(parse_with_prefix_generic::<i16>("0xFF").unwrap(), 255_i16);
    assert_eq!(parse_with_prefix_generic::<u128>("0xFF").unwrap(), 255_u128);

    assert_eq!(parse_hex("0xFF").unwrap(), 255);
    assert_eq!(parse_hex("F").unwrap(), 15);
    assert_eq!(parse_hex("100").unwrap(), 256);

    assert!(parse_hex("0x").is_err());
    assert!(parse_hex("0xZZZ").is_err());
    assert!(parse_hex("xyz").is_err());
}

use anyhow::{bail, ensure, Result};

fn parse_with_prefix(s: &str) -> Result<usize> {
    ensure!(s.len() > 2, "too short");

    let radix = match s.chars().nth(1).unwrap().to_ascii_lowercase() {
        'b' => 2,
        'o' => 8,
        'x' => 16,
        _ => bail!("unexpected radix"),
    };

    Ok(usize::from_str_radix(&s[2..], radix)?)
}

use num_traits::Num;

fn parse_with_prefix_generic<T>(s: &str) -> Result<T>
where
    T: Num,
    <T as Num>::FromStrRadixErr: std::error::Error + Send + Sync + 'static,
{
    ensure!(s.len() > 2, "too short");

    let radix = match s.chars().nth(1).unwrap().to_ascii_lowercase() {
        'b' => 2,
        'o' => 8,
        'x' => 16,
        _ => bail!("unexpected radix"),
    };

    Ok(<T as Num>::from_str_radix(&s[2..], radix)?)
}

fn parse_hex(s: &str) -> Result<usize, std::num::ParseIntError> {
    const RADIX: u32 = 16;
    const PREFIX_L: &str = "0x";
    const PREFIX_U: &str = "0X";

    if s.len() < 3 {
        usize::from_str_radix(s, RADIX)
    } else if s.starts_with(PREFIX_L) || s.starts_with(PREFIX_U) {
        usize::from_str_radix(&s[2..], RADIX)
    } else {
        usize::from_str_radix(s, RADIX)
    }
}
