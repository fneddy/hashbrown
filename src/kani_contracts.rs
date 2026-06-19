use crate::control::{Group, Tag};
use crate::raw::prev_pow2;

#[kani::proof_for_contract(Group::match_tag)]
fn contract_group_match_tag() {
    let word: u16 = kani::any();
    // Place 7 arbitrary bits in the top 7 positions so Tag::full produces
    // a tag with that exact byte value, covering the full space of full tags.
    let tag_bits: u8 = kani::any();
    kani::assume(tag_bits < 0x80);
    let tag = Tag::full((tag_bits as u64) << 57);
    let group = Group::from_u64_ne(word as u64);
    group.match_tag(tag);
}

#[kani::proof_for_contract(Group::match_empty)]
fn contract_group_match_empty() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    group.match_empty();
}

#[kani::proof_for_contract(Group::match_empty_or_deleted)]
fn contract_group_match_empty_or_deleted() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    group.match_empty_or_deleted();
}

#[kani::proof_for_contract(Group::match_full)]
fn contract_group_match_full() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    group.match_full();
}

#[kani::proof_for_contract(Group::convert_special_to_empty_and_full_to_deleted)]
fn contract_group_convert_special_to_empty_and_full_to_deleted() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    group.convert_special_to_empty_and_full_to_deleted();
}

#[cfg(not(feature = "nightly"))]
#[kani::proof_for_contract(crate::util::likely)]
fn contract_likely() {
    let b: bool = kani::any();
    crate::util::likely(b);
}

#[cfg(not(feature = "nightly"))]
#[kani::proof_for_contract(crate::util::unlikely)]
fn contract_unlikely() {
    let b: bool = kani::any();
    crate::util::unlikely(b);
}

#[kani::proof_for_contract(prev_pow2)]
fn contract_prev_pow2() {
    let z: usize = kani::any();
    kani::assume(z != 0);
    prev_pow2(z);
}
