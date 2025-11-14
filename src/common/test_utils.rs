pub(crate) fn assert_eq_cycle(a: Vec<usize>, b: Vec<usize>) {
    assert!(!a.is_empty());
    assert!(!b.is_empty());
    let mut offset = None;

    for i in 0..a.len() {
        if a[i] == b[0] {
            offset = Some(i);
            break;
        }
    }
    let offset = offset.unwrap();

    let a: Vec<usize> = a[offset..]
        .iter()
        .chain(a[..offset].iter())
        .cloned()
        .collect();

    assert_eq!(a, b);
}
