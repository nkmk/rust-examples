fn main() {
    let mut v: Vec<i32> = vec![100];

    assert_eq!(v.pop().unwrap_or_default(), 100);
    assert_eq!(v.pop().unwrap_or_default(), 0);

    let mut v: Vec<String> = vec![String::from("abc")];

    assert_eq!(v.pop().unwrap_or_default(), "abc");
    assert_eq!(v.pop().unwrap_or_default(), "");

    assert_eq!("100".parse::<i32>().unwrap_or_default(), 100);
    assert_eq!("abc".parse::<i32>().unwrap_or_default(), 0);

    let mut v: Vec<i32> = vec![100];

    assert_eq!(v.pop().unwrap_or(5), 100);
    assert_eq!(v.pop().unwrap_or(5), 5);

    assert_eq!("100".parse::<i32>().unwrap_or(5), 100);
    assert_eq!("abc".parse::<i32>().unwrap_or(5), 5);

    let mut v: Vec<String> = vec![String::from("abc")];

    assert_eq!(v.pop().unwrap_or_else(|| String::from("N/A")), "abc");
    assert_eq!(v.pop().unwrap_or_else(|| String::from("N/A")), "N/A");

    use std::fs;

    let path = "path/to/non_existent_file";

    assert_eq!(fs::read_to_string(path).unwrap_or_default(), "");
    assert_eq!(
        fs::read_to_string(path).unwrap_or_else(|_| String::from("N/A")),
        "N/A"
    );
}
