use crate::control::{Group, Tag};

#[kani::proof]
#[kani::solver(kissat)]
fn verify_tag_full_top_bit_clear_be() {
    let hash: u8 = kani::any();
    let tag = Tag::full(hash as u64);
    kani::assert(tag.is_full(), "Tag::full must always produce a full tag on BE");
    kani::assert(!tag.is_special(), "top bit must be clear on BE");
}

#[kani::proof]
#[kani::solver(kissat)]
#[kani::stub_verified(crate::control::Group::match_tag)]
#[kani::stub_verified(crate::control::Group::match_empty)]
#[kani::stub_verified(crate::control::Group::match_full)]
fn verify_match_empty_or_deleted_exact_be() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    let result = group.match_empty_or_deleted();
    let bytes = (word as u64).to_ne_bytes();
    for i in 0..Group::WIDTH {
        let has_high_bit = bytes[i] & 0x80 != 0;
        let bit_set = (result.0.to_le() >> (i * 8)) & 0xFF != 0;
        kani::assert(
            has_high_bit == bit_set,
            "match_empty_or_deleted must be exact on BE: bit set iff high bit set",
        );
    }
}

#[kani::proof]
#[kani::solver(kissat)]
#[kani::stub_verified(crate::control::Group::match_tag)]
#[kani::stub_verified(crate::control::Group::match_empty)]
#[kani::stub_verified(crate::control::Group::match_empty_or_deleted)]
fn verify_match_full_exact_be() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    let result = group.match_full();
    let bytes = (word as u64).to_ne_bytes();
    for i in 0..Group::WIDTH {
        let is_full = bytes[i] & 0x80 == 0;
        let bit_set = (result.0.to_le() >> (i * 8)) & 0xFF != 0;
        kani::assert(
            is_full == bit_set,
            "match_full must be exact on BE: bit set iff full",
        );
    }
}

#[kani::proof]
#[kani::solver(kissat)]
#[kani::stub_verified(crate::control::Group::match_tag)]
#[kani::stub_verified(crate::control::Group::match_empty)]
#[kani::stub_verified(crate::control::Group::match_empty_or_deleted)]
#[kani::stub_verified(crate::control::Group::match_full)]
fn verify_convert_special_to_empty_and_full_to_deleted_be() {
    let word: u16 = kani::any();
    let group = Group::from_u64_ne(word as u64);
    let result = group.convert_special_to_empty_and_full_to_deleted();
    let in_bytes = (word as u64).to_ne_bytes();
    let out_bytes = result.to_u64_ne().to_ne_bytes();
    for i in 0..Group::WIDTH {
        if in_bytes[i] & 0x80 != 0 {
            kani::assert(out_bytes[i] == Tag::EMPTY.0, "special -> EMPTY on BE");
        } else {
            kani::assert(out_bytes[i] == Tag::DELETED.0, "full -> DELETED on BE");
        }
    }
}

#[kani::proof]
#[kani::solver(kissat)]
#[kani::stub_verified(crate::control::Group::match_empty)]
#[kani::stub_verified(crate::control::Group::match_empty_or_deleted)]
#[kani::stub_verified(crate::control::Group::match_full)]
#[kani::stub_verified(crate::control::Group::convert_special_to_empty_and_full_to_deleted)]
fn verify_match_tag_no_false_negatives_be() {
    let word: u16 = kani::any();
    // Construct a full tag with an arbitrary 7-bit value by placing those
    // bits in the top 7 positions of a u64, which is where Tag::full reads from.
    let tag_bits: u8 = kani::any();
    kani::assume(tag_bits < 0x80);
    let tag = Tag::full((tag_bits as u64) << 57);
    let group = Group::from_u64_ne(word as u64);
    let result = group.match_tag(tag);
    let bytes = (word as u64).to_ne_bytes();
    // tag.0 is tag_bits (top 7 bits shifted down), compare against actual tag byte
    for i in 0..Group::WIDTH {
        if bytes[i] == tag_bits {
            let bit_set = (result.0.to_le() >> (i * 8)) & 0xFF != 0;
            kani::assert(bit_set, "match_tag must have no false negatives on BE");
        }
    }
}
