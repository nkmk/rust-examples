#[allow(clippy::needless_collect)]
fn main() {
    slice_examples();

    for i in 1..4 {
        println!("{i}");
    }
    // 1
    // 2
    // 3

    let v: Vec<i32> = (1..4).collect();
    assert_eq!(v, [1, 2, 3]);

    let v: Vec<i32> = (1..4).map(|x| x * 10).collect();
    assert_eq!(v, [10, 20, 30]);

    // let v: Vec<i32> = (1..).collect();
    // thread 'main' panicked at 'capacity overflow'

    let a = ['a', 'b', 'c'];
    let v: Vec<(i32, char)> = (10..).zip(a).collect();
    assert_eq!(v, [(10, 'a'), (11, 'b'), (12, 'c')]);

    // for i in ..4 {}
    // error[E0277]: `std::ops::RangeTo<{integer}>` is not an iterator

    // let v: Vec<i32> = (..4).collect();
    // error[E0599]: `std::ops::RangeTo<{integer}>` is not an iterator

    let v: Vec<i32> = (1..10).step_by(2).collect();
    assert_eq!(v, [1, 3, 5, 7, 9]);

    let v: Vec<i32> = (1..4).rev().collect();
    assert_eq!(v, [3, 2, 1]);

    let v: Vec<i32> = (1..10).step_by(3).rev().collect();
    assert_eq!(v, [7, 4, 1]);

    let v: Vec<i32> = (1..10).rev().step_by(3).collect();
    assert_eq!(v, [9, 6, 3]);

    // let v: Vec<i32> = (1..=10).step_by(3).rev().collect();
    // error[E0277]: the trait bound `std::ops::RangeInclusive<i32>: std::iter::ExactSizeIterator` is not satisfied

    let v: Vec<i32> = (1..=10).rev().step_by(3).collect();
    assert_eq!(v, [10, 7, 4, 1]);

    let start = 5;
    let end = 1;
    let r = start..end;

    assert!(r.is_empty());

    let v: Vec<i32> = r.collect();
    assert!(v.is_empty());

    let start: usize = 5;
    let end: usize = 1;
    let r = start..end;

    let a = [0, 10, 20, 30, 40];

    // let x = &a[r];
    // thread 'main' panicked at 'slice index starts at 5 but ends at 1'

    assert_eq!(a.get(r), None);
}

#[rustfmt::skip]
fn slice_examples() {
    let a = [0, 10, 20, 30, 40];

    assert_eq!(&a[1..3],  [   10, 20        ]); // Range
    assert_eq!(&a[1..=3], [   10, 20, 30    ]); // RangeInclusive
    assert_eq!(&a[..3],   [0, 10, 20        ]); // RangeTo
    assert_eq!(&a[..=3],  [0, 10, 20, 30    ]); // RangeToInclusive
    assert_eq!(&a[1..],   [   10, 20, 30, 40]); // RangeFrom
    assert_eq!(&a[..],    [0, 10, 20, 30, 40]); // RangeFull

    // let x = &a[1..100];
    // thread 'main' panicked at 'range end index 100 out of range for slice of length 5'

    assert_eq!(a.get(1..3).unwrap(), &a[1..3]);
    assert_eq!(a.get(1..100), None);
}
