#[allow(clippy::or_fun_call)]
fn main() {
    fn test_func(i: i32) -> String {
        println!("Print from function: No.{i}");
        String::from("N/A")
    }

    let mut v: Vec<String> = vec![String::from("abc")];

    assert_eq!(v.pop().unwrap_or(test_func(0)), "abc");
    // Print from function: No.0

    assert_eq!(v.pop().unwrap_or(test_func(1)), "N/A");
    // Print from function: No.1

    let mut v: Vec<String> = vec![String::from("abc")];

    assert_eq!(v.pop().unwrap_or_else(|| test_func(2)), "abc");

    assert_eq!(v.pop().unwrap_or_else(|| test_func(3)), "N/A");
    // Print from function: No.3
}
