fn main() {
    let v: Vec<i32> = vec![100, 200, 300];
    let a: [i32; 3] = [10, 20, 30];

    print_first_element_vec(&v);
    // 100

    // print_first_element_vec(&v[1..]);
    // error[E0308]: mismatched types

    // print_first_element_vec(&a);
    // error[E0308]: mismatched types

    print_first_element_slice(&v);
    // 100

    print_first_element_slice(&v[1..]);
    // 200

    print_first_element_slice(&a);
    // 10

    let mut v: Vec<i32> = vec![100, 200, 300];
    let mut a: [i32; 3] = [10, 20, 30];

    first_element_to_zero(&mut v);
    assert_eq!(v, [0, 200, 300]);

    first_element_to_zero(&mut a);
    assert_eq!(a, [0, 20, 30]);

    let mut v: Vec<i32> = vec![100, 200, 300];

    push_zero(&mut v);
    assert_eq!(v, [100, 200, 300, 0]);

    let s_str: &str = "abc";
    let s_string: String = String::from("xyz");

    print_first_char(s_str);
    // a

    print_first_char(&s_string);
    // x

    print_first_char(&s_str[1..]);
    // b

    print_first_char(&s_string[1..]);
    // y

    let mut s_string: String = String::from("xyz");

    push_str_abc(&mut s_string);
    assert_eq!(s_string, "xyz_abc");
}

#[allow(clippy::ptr_arg)]
fn print_first_element_vec(v: &Vec<i32>) {
    println!("{}", v[0]);
}

fn print_first_element_slice(s: &[i32]) {
    println!("{}", s[0]);
}

fn first_element_to_zero(s: &mut [i32]) {
    s[0] = 0;
}

fn push_zero(s: &mut Vec<i32>) {
    s.push(0);
}

fn print_first_char(s: &str) {
    println!("{}", s.chars().next().unwrap());
}

fn push_str_abc(s: &mut String) {
    s.push_str("_abc");
}
