use anyhow::{anyhow, bail, ensure, Result};

fn main() {
    fn with_anyhow(i: i32) -> Result<i32> {
        if i < 10 {
            return Err(anyhow!("Value must be at least 10, got {}", i));
        };
        Ok(i)
    }

    assert_eq!(with_anyhow(10).unwrap(), 10);
    assert_eq!(
        with_anyhow(9).unwrap_err().to_string(),
        "Value must be at least 10, got 9"
    );

    fn with_bail(i: i32) -> Result<i32> {
        if i < 10 {
            bail!("Value must be at least 10, got {}", i);
        };
        Ok(i)
    }

    assert_eq!(with_bail(10).unwrap(), 10);
    assert_eq!(
        with_bail(9).unwrap_err().to_string(),
        "Value must be at least 10, got 9"
    );

    fn with_ensure(i: i32) -> Result<i32> {
        ensure!(i >= 10, "Value must be at least 10, got {}", i);
        Ok(i)
    }

    assert_eq!(with_ensure(10).unwrap(), 10);
    assert_eq!(
        with_ensure(9).unwrap_err().to_string(),
        "Value must be at least 10, got 9"
    );
}
