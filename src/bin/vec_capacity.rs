fn main() {
    let mut v = vec![0, 1, 2];
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 3);

    v.push(3);
    assert_eq!(v, [0, 1, 2, 3]);
    assert_eq!(v.len(), 4);
    assert_eq!(v.capacity(), 6);

    let mut v = Vec::with_capacity(10);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 10);

    v.push(0);
    v.push(1);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 10);

    let mut v = vec![0, 1, 2];
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 3);

    v.reserve(10);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 13);

    v.reserve(5);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 13);

    let mut v = vec![0, 1, 2];
    v.reserve(1);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 6);

    let mut v = vec![0, 1, 2];
    v.reserve_exact(1);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![0, 1, 2];
    let result = v.try_reserve(10);
    assert!(result.is_ok());
    assert_eq!(v.capacity(), 13);

    let mut v = vec![0, 1, 2];
    let result = v.try_reserve(isize::MAX as usize);
    assert!(result.is_err());
    assert_eq!(v.capacity(), 3);

    let mut v = vec![0, 1, 2];
    v.reserve(10);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 13);

    v.shrink_to_fit();
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 3);

    let mut v = vec![0, 1, 2];
    v.reserve(10);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 13);

    v.shrink_to(10);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 10);

    v.shrink_to(0);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 3);
}
