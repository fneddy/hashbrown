//! Pointer-key stress tests.

#![cfg(not(miri))]

use hashbrown::HashMap;

#[test]
fn pointer_keys_match_values_after_large_insert() {
    let len = 1 << 20;
    let data = vec![0_u64; len];
    let base = data.as_ptr();

    let mut map = HashMap::with_capacity(len);

    for index in 0..len {
        let ptr = base.wrapping_add(index);
        map.insert(ptr, ptr);
    }

    assert_eq!(map.len(), len);

    for (key, value) in &map {
        assert_eq!(*key, *value);
    }
}

#[test]
fn pointer_keys_match_values_after_churn() {
    let len = 1 << 18;
    let data = vec![0_u64; len];
    let base = data.as_ptr();

    let mut map = HashMap::with_capacity(len);

    for index in 0..len {
        let ptr = base.wrapping_add(index);
        map.insert(ptr, ptr);
    }

    for index in (0..len).step_by(3) {
        let ptr = base.wrapping_add(index);
        assert_eq!(map.remove(&ptr), Some(ptr));
    }

    for index in (0..len).step_by(3) {
        let ptr = base.wrapping_add(index);
        map.insert(ptr, ptr);
    }

    assert_eq!(map.len(), len);

    for (key, value) in &map {
        assert_eq!(*key, *value);
    }
}
