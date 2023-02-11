fn main() {
    let a = 100;

    println!("{a}");
    // 100

    dbg!(a);
    // [src/bin/dbg.rs:7] a = 100

    let s = String::from("abc");

    dbg!(s);
    // [src/bin/dbg.rs:12] s = "abc"

    // let x = s.len();
    // error[E0382]: borrow of moved value: `s`

    let s = String::from("abc");

    dbg!(&s);
    // [src/bin/dbg.rs:20] &s = "abc"

    let x = s.len();
    assert_eq!(x, 3);

    // https://doc.rust-lang.org/std/macro.dbg.html
    let a = 2;
    let b = dbg!(a * 2) + 1;
    // [src/bin/dbg.rs:28] a * 2 = 4

    assert_eq!(b, 5);

    fn factorial(n: u32) -> u32 {
        if dbg!(n <= 1) {
            dbg!(1)
        } else {
            dbg!(n * factorial(n - 1))
        }
    }

    dbg!(factorial(4));
    // [src/bin/dbg.rs:34] n <= 1 = false
    // [src/bin/dbg.rs:34] n <= 1 = false
    // [src/bin/dbg.rs:34] n <= 1 = false
    // [src/bin/dbg.rs:34] n <= 1 = true
    // [src/bin/dbg.rs:35] 1 = 1
    // [src/bin/dbg.rs:37] n * factorial(n - 1) = 2
    // [src/bin/dbg.rs:37] n * factorial(n - 1) = 6
    // [src/bin/dbg.rs:37] n * factorial(n - 1) = 24
    // [src/bin/dbg.rs:41] factorial(4) = 24
}
