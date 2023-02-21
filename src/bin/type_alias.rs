#[allow(unused_variables, dead_code, non_camel_case_types)]
fn main() {
    type Byte = u8;

    let b: Byte = 100;
    let u: u8 = 100;

    type byte = u8;
    // warning: type `byte` should have an upper camel case name

    let b: byte = 100;

    type Point = (u8, u8);

    let p: Point = (100, 200);

    assert_eq!(p.0, 100);
    assert_eq!(p.1, 200);

    type MyString = String;

    let s = MyString::from("abc");

    struct MyStruct {
        x: i32,
    }
    type MyStructAlias = MyStruct;

    let my_struct = MyStructAlias { x: 100 };

    struct MyTupleStruct(u8);
    type MyTupleStructAlias = MyTupleStruct;

    // let my_tuple_struct = MyTupleStructAlias(100);
    // error[E0423]: expected function, tuple struct or tuple variant, found type alias `MyTupleStructAlias`
}
