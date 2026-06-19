use crate::control::{Group, Tag};

#[kani::proof]
fn verify_tag_full_top_bit_clear_be() {
    let hash: u64 = kani::any();
    let tag = Tag::full(hash);
    kani::assert(tag.is_full(), "Tag::full must always produce a full tag on BE");
    kani::assert(tag.0 & 0x80 == 0, "top bit must be clear on BE");
}

#[kani::proof]
fn verify_match_empty_or_deleted_exact_be() {
    let word: u64 = kani::any();
    let group = Group::from_u64_ne(word);
    let result = group.match_empty_or_deleted();
    let bytes = word.to_ne_bytes();
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
fn verify_match_full_exact_be() {
    let word: u64 = kani::any();
    let group = Group::from_u64_ne(word);
    let result = group.match_full();
    let bytes = word.to_ne_bytes();
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
fn verify_convert_special_to_empty_and_full_to_deleted_be() {
    let word: u64 = kani::any();
    let group = Group::from_u64_ne(word);
    let result = group.convert_special_to_empty_and_full_to_deleted();
    let in_bytes = word.to_ne_bytes();
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
fn verify_match_tag_no_false_negatives_be() {
    let word: u64 = kani::any();
    let tag_byte: u8 = kani::any();
    kani::assume(tag_byte & 0x80 == 0);
    let tag = Tag(tag_byte);
    let group = Group::from_u64_ne(word);
    let result = group.match_tag(tag);
    let bytes = word.to_ne_bytes();
    for i in 0..Group::WIDTH {
        if bytes[i] == tag_byte {
            let bit_set = (result.0.to_le() >> (i * 8)) & 0xFF != 0;
            kani::assert(bit_set, "match_tag must have no false negatives on BE");
        }
    }
}
