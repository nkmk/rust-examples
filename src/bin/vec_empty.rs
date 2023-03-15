#[allow(clippy::vec_init_then_push, unused_variables)]
fn main() {
    let mut v = Vec::new();
    v.push(100); // Vec<i32>

    let mut v = Vec::new();
    v.push(String::from("abc")); // Vec<String>

    let v = Vec::<u8>::new();
    let v: Vec<u8> = Vec::new();

    // let v = Vec::new();
    // error[E0282]: type annotations needed for `std::vec::Vec<T>`

    let v: Vec<u8> = Vec::new();
    assert_eq!(v.capacity(), 0);

    let v: Vec<u8> = Vec::with_capacity(100);
    assert_eq!(v.capacity(), 100);

    let v = Vec::<u8>::with_capacity(100);

    let mut v = Vec::with_capacity(100);
    v.push(100); // Vec<i32>

    let mut v = Vec::with_capacity(100);
    v.push(String::from("abc")); // Vec<String>

    // let v = Vec::with_capacity(100);
    // error[E0282]: type annotations needed for `std::vec::Vec<T>`

    let mut v = vec![];
    v.push(100); // Vec<i32>

    let mut v = vec![];
    v.push(String::from("abc")); // Vec<String>

    let v: Vec<u8> = vec![];

    // let v = vec![];
    // error[E0282]: type annotations needed for `std::vec::Vec<T>`

    let v = vec![true; 0]; // Vec<bool>
    let v = vec![false; 0]; // Vec<bool>

    let v = vec![String::from("abc"); 0]; // Vec<String>
}
