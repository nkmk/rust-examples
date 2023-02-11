fn main() {
    let v: Vec<String> = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ];
    let s: String = v.concat();
    assert_eq!(s, "aaabbbccc");

    let s: String = v[1..].concat();
    assert_eq!(s, "bbbccc");

    let a: [&str; 3] = ["aaa", "bbb", "ccc"];
    let s: String = a.concat();
    assert_eq!(s, "aaabbbccc");

    let v: Vec<String> = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ];
    let s: String = v.join("-");
    assert_eq!(s, "aaa-bbb-ccc");

    let sep = String::from("+");
    let s: String = v.join(&sep);
    assert_eq!(s, "aaa+bbb+ccc");

    let a: [&str; 3] = ["aaa", "bbb", "ccc"];
    let s: String = a.join("-");
    assert_eq!(s, "aaa-bbb-ccc");

    let v: Vec<String> = vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ];

    let s: String = v
        .iter()
        .map(|s| s.to_uppercase())
        .collect::<Vec<String>>()
        .join("x");
    assert_eq!(s, "AAAxBBBxCCC");

    use itertools::Itertools;

    let s = v.iter().map(|s| s.to_uppercase()).join("x");
    assert_eq!(s, "AAAxBBBxCCC");

    let s = v.iter().map(|s| s.to_uppercase()).join("");
    assert_eq!(s, "AAABBBCCC");
}
