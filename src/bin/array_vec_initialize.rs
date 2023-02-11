#[allow(unused_variables)]
fn main() {
    let a = [0, 10, 20];
    assert_eq!(a, [0, 10, 20]);

    let a: [i32; 3] = [0, 10, 20];
    assert_eq!(a, [0, 10, 20]);

    let a = [0_i32, 10, 20];
    assert_eq!(a, [0, 10, 20]);

    let v = vec![0, 10, 20];
    assert_eq!(v, [0, 10, 20]);

    let v: Vec<i32> = vec![0, 10, 20];
    assert_eq!(v, [0, 10, 20]);

    let v = vec![0_i32, 10, 20];
    assert_eq!(v, [0, 10, 20]);

    let a = [10; 3];
    assert_eq!(a, [10, 10, 10]);

    let a: [i32; 3] = [10; 3];
    assert_eq!(a, [10, 10, 10]);

    let a = [10_i32; 3];
    assert_eq!(a, [10, 10, 10]);

    let v = vec![10; 3];
    assert_eq!(v, [10, 10, 10]);

    let v: Vec<i32> = vec![10; 3];
    assert_eq!(v, [10, 10, 10]);

    let v = vec![10_i32; 3];
    assert_eq!(v, [10, 10, 10]);

    let length = 3;
    // let a = [10; length];
    // error[E0435]: attempt to use a non-constant value in a constant
    // ---------- help: consider using `const` instead of `let`: `const length`

    const LENGTH: usize = 3;
    let a = [10; LENGTH];
    assert_eq!(a, [10, 10, 10]);

    let length = 3;
    let v = vec![10; length];
    assert_eq!(v, [10, 10, 10]);

    let a: [&str; 3] = ["abc"; 3];
    assert_eq!(a, ["abc", "abc", "abc"]);

    // let a: [String; 3] = [String::from("abc"); 3];
    // error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
    // note: the `Copy` trait is required because this value will be copied for each element of the array

    let a: [String; 3] = [
        String::from("abc"),
        String::from("abc"),
        String::from("abc"),
    ];
    assert_eq!(a, ["abc", "abc", "abc"]);

    let v: Vec<String> = vec![String::from("abc"); 3];
    assert_eq!(v, ["abc", "abc", "abc"]);

    let s = String::from("abc");
    let v: Vec<String> = vec![s; 3];
    assert_eq!(v, ["abc", "abc", "abc"]);
    // assert_eq!(s, "abc");
    // error[E0382]: borrow of moved value: `s`

    let s = String::from("abc");
    let v: Vec<String> = vec![s.clone(); 3];
    assert_eq!(v, ["abc", "abc", "abc"]);
    assert_eq!(s, "abc");

    let a: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
    assert_eq!(a, [[1, 2], [3, 4], [5, 6]]);

    let a: [[i32; 2]; 3] = [[10; 2]; 3];
    assert_eq!(a, [[10, 10], [10, 10], [10, 10]]);

    let v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    assert_eq!(v, [[1, 2], [3, 4], [5, 6]]);

    let v: Vec<Vec<i32>> = vec![vec![10; 2]; 3];
    assert_eq!(v, [[10, 10], [10, 10], [10, 10]]);

    skip_fmt();
}

#[rustfmt::skip]
fn skip_fmt() {
    let a: [[[i32; 2]; 3]; 2] = [[[10; 2]; 3]; 2];
    assert_eq!(
        a,
        [[[10, 10], [10, 10], [10, 10]],
         [[10, 10], [10, 10], [10, 10]]]
    );

    let v: Vec<Vec<Vec<i32>>> = vec![vec![vec![10; 2]; 3]; 2];
    assert_eq!(
        v,
        [[[10, 10], [10, 10], [10, 10]],
         [[10, 10], [10, 10], [10, 10]]]
    );
}
