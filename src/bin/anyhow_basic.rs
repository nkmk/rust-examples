use anyhow::{ensure, Context, Result};
use std::fs;

fn main() {
    fs::create_dir_all("data/temp").unwrap();

    let path_123 = "data/temp/123.txt";
    fs::write(path_123, "123").unwrap();

    let path_abc = "data/temp/abc.txt";
    fs::write(path_abc, "abc").unwrap();

    let path_non_existent = "path/to/non_existent.txt";

    fn with_unwrap(path: &str) -> i32 {
        let s = fs::read_to_string(path).unwrap();
        let i = s.parse::<i32>().unwrap();

        i * 10
    }

    assert_eq!(with_unwrap(path_123), 1230);

    // let x = with_unwrap(path_abc);
    // thread 'main' panicked at 'called `Result::unwrap()` ...

    // let x = with_unwrap(path_non_existent);
    // thread 'main' panicked at 'called `Result::unwrap()` ...

    fn with_anyhow(path: &str) -> Result<i32> {
        let s = fs::read_to_string(path)?;
        let i = s.parse::<i32>()?;

        Ok(i * 10)
    }

    assert_eq!(with_anyhow(path_123).unwrap(), 1230);

    println!("{:?}", with_anyhow(path_abc).unwrap_err());
    // invalid digit found in string

    println!("{:?}", with_anyhow(path_non_existent).unwrap_err());
    // No such file or directory (os error 2)

    fn with_anyhow_context(path: &str) -> Result<i32> {
        let s = fs::read_to_string(path).with_context(|| format!("Failed to read {path:?}"))?;
        let i = s
            .parse::<i32>()
            .with_context(|| format!("Failed to parse {s:?}"))?;

        Ok(i * 10)
    }

    assert_eq!(with_anyhow_context(path_123).unwrap(), 1230);

    println!("{:?}", with_anyhow_context(path_abc).unwrap_err());
    // Failed to parse "abc"
    //
    // Caused by:
    //     invalid digit found in string

    println!("{:?}", with_anyhow_context(path_non_existent).unwrap_err());
    // Failed to read "path/to/non_existent.txt"
    //
    // Caused by:
    //     No such file or directory (os error 2)

    fn with_anyhow_option(path: &str) -> Result<i32> {
        let s = fs::read_to_string(path).with_context(|| format!("Failed to read {path:?}"))?;
        let i = s
            .parse::<i32>()
            .with_context(|| format!("Failed to parse {s:?}"))?;
        let i = i.checked_add(100).context("Overflow occurred")?;

        Ok(i)
    }

    let path_max = "data/temp/max.txt";
    fs::write(path_max, i32::MAX.to_string()).unwrap();

    println!("{:?}", with_anyhow_option(path_max).unwrap_err());
    // Overflow occurred

    assert_eq!(with_anyhow_option(path_123).unwrap(), 223);
    assert!(with_anyhow_option(path_abc).is_err());
    assert!(with_anyhow_option(path_non_existent).is_err());

    fn with_anyhow_macro(path: &str) -> Result<i32> {
        let s = fs::read_to_string(path).with_context(|| format!("Failed to read {path:?}"))?;
        let i = s
            .parse::<i32>()
            .with_context(|| format!("Failed to parse {s:?}"))?;
        let i = i.checked_add(100).context("Overflow occurred")?;

        ensure!(i >= 1000, "Value must be at least 1000, got {}", i);

        Ok(i)
    }

    println!("{:?}", with_anyhow_macro(path_123).unwrap_err());
    // Value must be at least 1000, got 223

    assert!(with_anyhow_option(path_abc).is_err());
    assert!(with_anyhow_option(path_non_existent).is_err());
    assert!(with_anyhow_option(path_max).is_err());

    fs::remove_file(path_123).unwrap();
    fs::remove_file(path_abc).unwrap();
    fs::remove_file(path_max).unwrap();
}
