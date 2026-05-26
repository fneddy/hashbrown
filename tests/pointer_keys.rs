//! Pointer-key stress tests.

#![cfg(not(miri))]

use hashbrown::HashMap;
use std::ptr::NonNull;

fn make_ptr(index: usize) -> *const u8 {
    NonNull::<u8>::dangling().as_ptr().wrapping_add(index)
}

#[test]
fn pointer_keys_match_values_after_large_insert() {
    let len = 1 << 20;
    let mut map = HashMap::with_capacity(len);

    for index in 0..len {
        let ptr = make_ptr(index);
        assert_eq!(map.insert(ptr, ptr), None);
    }

    assert_eq!(map.len(), len);

    for (key, value) in &map {
        assert_eq!(*key, *value);
    }
}

#[test]
fn pointer_keys_match_values_after_churn() {
    let len = 1 << 18;
    let mut map = HashMap::with_capacity(len);

    for index in 0..len {
        let ptr = make_ptr(index);
        assert_eq!(map.insert(ptr, ptr), None);
    }

    for index in (0..len).step_by(3) {
        let ptr = make_ptr(index);
        assert_eq!(map.remove(&ptr), Some(ptr));
    }

    for index in (0..len).step_by(3) {
        let ptr = make_ptr(index);
        assert_eq!(map.insert(ptr, ptr), None);
    }

    assert_eq!(map.len(), len);

    for (key, value) in &map {
        assert_eq!(*key, *value);
    }
}
