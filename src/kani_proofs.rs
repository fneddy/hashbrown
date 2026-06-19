use crate::HashMap;

#[kani::proof]
fn verify_new_map_is_empty() {
    let map: HashMap<u8, u8> = HashMap::new();
    kani::assert(map.is_empty(), "new map must be empty");
    kani::assert(map.len() == 0, "new map len must be 0");
    kani::assert(map.capacity() == 0, "new map capacity must be 0");
}

#[kani::proof]
fn verify_insert_increases_len() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    kani::assert(map.len() == 1, "len must be 1 after inserting one element");
    kani::assert(!map.is_empty(), "map must not be empty after insert");
}

#[kani::proof]
fn verify_insert_same_key_does_not_grow_len() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v1: u8 = kani::any();
    let v2: u8 = kani::any();
    map.insert(k, v1);
    map.insert(k, v2);
    kani::assert(map.len() == 1, "inserting same key twice must keep len == 1");
}

#[kani::proof]
fn verify_get_after_insert() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    let got = map.get(&k);
    kani::assert(got == Some(&v), "get must return inserted value");
}

#[kani::proof]
fn verify_get_missing_key_returns_none() {
    let map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    kani::assert(map.get(&k).is_none(), "get on empty map must return None");
}

#[kani::proof]
fn verify_remove_decreases_len() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    let removed = map.remove(&k);
    kani::assert(removed == Some(v), "remove must return old value");
    kani::assert(map.len() == 0, "len must be 0 after removing only element");
    kani::assert(map.is_empty(), "map must be empty after removing only element");
}

#[kani::proof]
fn verify_remove_missing_key_returns_none() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    kani::assert(
        map.remove(&k).is_none(),
        "remove on empty map must return None",
    );
}

#[kani::proof]
fn verify_contains_key_after_insert() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    kani::assert(map.contains_key(&k), "contains_key must be true after insert");
}

#[kani::proof]
fn verify_contains_key_false_after_remove() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    map.remove(&k);
    kani::assert(
        !map.contains_key(&k),
        "contains_key must be false after remove",
    );
}

#[kani::proof]
fn verify_insert_overwrites_value() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v1: u8 = kani::any();
    let v2: u8 = kani::any();
    map.insert(k, v1);
    map.insert(k, v2);
    kani::assert(map.get(&k) == Some(&v2), "second insert must overwrite first");
}

#[kani::proof]
fn verify_two_distinct_keys() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k1: u8 = kani::any();
    let k2: u8 = kani::any();
    kani::assume(k1 != k2);
    let v1: u8 = kani::any();
    let v2: u8 = kani::any();
    map.insert(k1, v1);
    map.insert(k2, v2);
    kani::assert(map.len() == 2, "two distinct keys must give len == 2");
    kani::assert(map.get(&k1) == Some(&v1), "k1 must map to v1");
    kani::assert(map.get(&k2) == Some(&v2), "k2 must map to v2");
}

#[kani::proof]
fn verify_clear_empties_map() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    map.clear();
    kani::assert(map.is_empty(), "map must be empty after clear");
    kani::assert(map.len() == 0, "len must be 0 after clear");
    kani::assert(
        map.get(&k).is_none(),
        "get must return None after clear",
    );
}

#[kani::proof]
fn verify_capacity_at_least_len() {
    let mut map: HashMap<u8, u8> = HashMap::new();
    let k: u8 = kani::any();
    let v: u8 = kani::any();
    map.insert(k, v);
    kani::assert(
        map.capacity() >= map.len(),
        "capacity must be >= len at all times",
    );
}

#[kani::proof]
#[kani::unwind(2)]
fn verify_with_capacity_has_sufficient_room() {
    let cap: usize = kani::any();
    kani::assume(cap <= 8);
    let map: HashMap<u8, u8> = HashMap::with_capacity(cap);
    kani::assert(
        map.capacity() >= cap,
        "with_capacity must provide at least the requested capacity",
    );
    kani::assert(map.is_empty(), "freshly created map must be empty");
}
