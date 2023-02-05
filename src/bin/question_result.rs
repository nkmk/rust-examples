#[allow(unused_variables)]
fn main() {
    fn parse_with_unwrap(s: &str) -> i32 {
        let i = s.parse::<i32>().unwrap();
        i * 2
    }

    assert_eq!(parse_with_unwrap("10"), 20);

    // let x = parse_with_unwrap("abc");
    // thread 'main' panicked at 'called `Result::unwrap()` ...

    fn parse_with_match(s: &str) -> Result<i32, std::num::ParseIntError> {
        let i = match s.parse::<i32>() {
            Ok(i) => i,
            Err(e) => return Err(e),
        };
        Ok(i * 2)
    }

    assert_eq!(parse_with_match("10").unwrap(), 20);
    assert!(parse_with_match("abc").is_err());

    fn parse_with_question(s: &str) -> Result<i32, std::num::ParseIntError> {
        let i = s.parse::<i32>()?;
        Ok(i * 2)
    }

    assert_eq!(parse_with_question("10").unwrap(), 20);
    assert!(parse_with_question("abc").is_err());

    use std::fs;

    fn read_with_question(path: &str) -> Result<(), std::io::Error> {
        let s: String = fs::read_to_string(path)?;
        // do something
        Ok(())
    }

    let non_existent_path = "path/to/non_existent_file";

    assert!(read_with_question(non_existent_path).is_err());
}
