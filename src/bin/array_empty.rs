#[allow(unused_variables)]
fn main() {
    let a: [i32; 0] = [];
    let a: [u8; 0] = [];

    // let a = [];
    // error[E0282]: type annotations needed for `[_; 0]`

    let a = [100; 0]; // [i32; 0]
    let a = [100_u8; 0]; // [u8; 0]
}
