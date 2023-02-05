use anyhow::{Context, Result};

fn main() {
    fn pop_ok_or(v: &mut Vec<&str>) -> Result<String, &'static str> {
        let s = v.pop().ok_or("Vector is empty")?;
        Ok(String::from(s))
    }

    let mut v = vec!["10", "20"];
    assert_eq!(pop_ok_or(&mut v).unwrap(), "20");

    let mut v = vec![];
    assert_eq!(pop_ok_or(&mut v).unwrap_err(), "Vector is empty");

    fn pop_parse_anyhow(v: &mut Vec<&str>) -> Result<i32> {
        let s = v.pop().context("Vector is empty")?;
        let i = s.parse::<i32>()?;
        Ok(i)
    }

    let mut v = vec!["10", "20"];
    assert_eq!(pop_parse_anyhow(&mut v).unwrap(), 20);

    let mut v = vec![];
    assert_eq!(
        pop_parse_anyhow(&mut v).unwrap_err().to_string(),
        "Vector is empty"
    );

    let mut v = vec!["abc", "xyz"];
    assert_eq!(
        pop_parse_anyhow(&mut v).unwrap_err().to_string(),
        "invalid digit found in string"
    );
}
