fn main() {
    let mut v: Vec<i32> = vec![0, 10, 20];
    assert_eq!(with_question(&mut v).unwrap(), "120");

    let mut v: Vec<i32> = vec![];
    assert_eq!(with_question(&mut v), None);

    let mut v: Vec<i32> = vec![i32::MAX];
    assert_eq!(with_question(&mut v), None);
}

fn with_question(v: &mut Vec<i32>) -> Option<String> {
    let i = v.pop()?;
    let i = i.checked_add(100)?;

    Some(i.to_string())
}

fn _with_question(v: &mut Vec<i32>) -> Option<String> {
    Some(v.pop()?.checked_add(100)?.to_string())
}
