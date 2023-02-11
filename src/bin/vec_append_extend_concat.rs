fn main() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![4, 5, 6];

    v1.append(&mut v2);

    assert_eq!(v1, [1, 2, 3, 4, 5, 6]);
    assert!(v2.is_empty());

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.extend(v2);

    assert_eq!(v1, [1, 2, 3, 4, 5, 6]);

    // v2;
    // error[E0382]: use of moved value: `v2`

    let mut v = vec![1, 2, 3];
    let a = [4, 5, 6];

    v.extend(a);

    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.extend(&v2[1..]);

    assert_eq!(v1, [1, 2, 3, 5, 6]);

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.extend(v2.iter().map(|x| x * 10));

    assert_eq!(v1, [1, 2, 3, 40, 50, 60]);

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.extend(v2.clone());

    assert_eq!(v1, [1, 2, 3, 4, 5, 6]);
    assert_eq!(v2, [4, 5, 6]);

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.extend(&v2);

    assert_eq!(v1, [1, 2, 3, 4, 5, 6]);
    assert_eq!(v2, [4, 5, 6]);

    let mut v1 = vec![String::from('a'), String::from('b'), String::from('c')];
    let v2 = vec![String::from('d'), String::from('e'), String::from('f')];

    // v1.extend(&v2);
    // error[E0271]: type mismatch resolving ...

    v1.extend_from_slice(&v2);

    assert_eq!(v1, ["a", "b", "c", "d", "e", "f"]);
    assert_eq!(v2, ["d", "e", "f"]);

    let mut v = vec![String::from('a'), String::from('b'), String::from('c')];
    let a = [String::from('d'), String::from('e'), String::from('f')];

    v.extend_from_slice(&a);

    assert_eq!(v, ["a", "b", "c", "d", "e", "f"]);
    assert_eq!(a, ["d", "e", "f"]);

    let mut v1 = vec![String::from('a'), String::from('b'), String::from('c')];
    let v2 = vec![String::from('d'), String::from('e'), String::from('f')];

    v1.extend_from_slice(&v2[1..]);

    assert_eq!(v1, ["a", "b", "c", "e", "f"]);
    assert_eq!(v2, ["d", "e", "f"]);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];

    let v = [v1, v2, v3].concat();
    assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

    // v1;
    // error[E0382]: use of moved value: `v1`

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];

    // let v = [&v1, &v2, &v3].concat();
    // error[E0599]: the method `concat` exists for array `[&std::vec::Vec<{integer}>; 3]`, but its trait bounds were not satisfied

    let v = [&v1[..], &v2[..], &v3[..]].concat();

    assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v2, [4, 5, 6]);
    assert_eq!(v3, [7, 8, 9]);

    let v = [&v1[1..], &v2[..], &v3[..2]].concat();

    assert_eq!(v, [2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v2, [4, 5, 6]);
    assert_eq!(v3, [7, 8, 9]);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];

    // let v = [v1, v2, v3].join(100);
    // error[E0277]: the trait bound `[std::vec::Vec<{integer}>]: std::slice::Join<{integer}>` is not satisfied

    let v = [v1, v2, v3].join(&100);
    assert_eq!(v, [1, 2, 3, 100, 4, 5, 6, 100, 7, 8, 9]);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];

    // let v = [v1, v2, v3].join(&[100, 200]);
    // error[E0277]: the trait bound `std::vec::Vec<{integer}>: std::borrow::Borrow<[[{integer}; 2]]>` is not satisfied

    let v = [v1, v2, v3].join(&[100, 200][..]);
    assert_eq!(v, [1, 2, 3, 100, 200, 4, 5, 6, 100, 200, 7, 8, 9]);

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];

    let v = [&v1[1..], &v2[..], &v3[..2]].join(&100);

    assert_eq!(v, [2, 3, 100, 4, 5, 6, 100, 7, 8]);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v2, [4, 5, 6]);
    assert_eq!(v3, [7, 8, 9]);
}
