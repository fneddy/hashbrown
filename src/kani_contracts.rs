use crate::control::{Group, Tag};
use crate::raw::prev_pow2;
use crate::util::{likely, unlikely};

#[kani::proof_for_contract(Group::match_tag)]
fn contract_group_match_tag() {
    let word: u16 = kani::any();
    let tag_byte: u8 = kani::any();
    let tag = Tag(tag_byte);
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

#[kani::proof_for_contract(likely)]
fn contract_likely() {
    let b: bool = kani::any();
    likely(b);
}

#[kani::proof_for_contract(unlikely)]
fn contract_unlikely() {
    let b: bool = kani::any();
    unlikely(b);
}

#[kani::proof_for_contract(prev_pow2)]
fn contract_prev_pow2() {
    let z: usize = kani::any();
    kani::assume(z != 0);
    prev_pow2(z);
}
