#[allow(unused_variables, clippy::used_underscore_binding)]
fn main() {
    let x = 100;
    // warning: unused variable: `x`
    // help: if this is intentional, prefix it with an underscore: `_x`

    let _x = 100;
    assert_eq!(_x, 100);

    let _ = 100;
    // assert_eq!(_, 100);
    // error: no rules expected the token `_`

    fn return_multi() -> (bool, i32, f64) {
        (true, 100, 0.123)
    }

    let (_, x, _) = return_multi();
    assert_eq!(x, 100);

    let a = String::from("abc");
    let _x = a;
    // assert_eq!(a, "abc");
    // error[E0382]: borrow of moved value: `a`
    assert_eq!(_x, "abc");

    let a = String::from("abc");
    let _ = a;
    assert_eq!(a, "abc");

    // https://users.rust-lang.org/t/what-are-pipes-and-underscores-doing-in-rust/14601/16
    struct D(i32);
    impl Drop for D {
        fn drop(&mut self) {
            println!("dropped {}", self.0);
        }
    }

    {
        let _x = D(0);
        let _ = D(1);
        println!("------");
    }
    // dropped 1
    // ------
    // dropped 0

    println!();

    {
        let a = D(0);
        let _ = a;
        println!("------");
    }
    // ------
    // dropped 0

    println!();

    {
        let a = D(0);
        let (x, _) = (100, a);
        println!("------");
    }
    // dropped 0
    // ------

    println!();

    fn test_func(_: String) {}

    let a = String::from("abc");
    test_func(a);
    // assert_eq!(a, "abc");
    // error[E0382]: borrow of moved value: `a`

    fn test_func_d(_: D) {
        println!("------");
    }

    {
        let x = D(0);
        test_func_d(x);
        println!("======");
    }
    // ------
    // dropped 0
    // ======
}
