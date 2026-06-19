use core::{fmt, mem};

/// Single tag in a control group.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct Tag(pub(super) u8);
impl Tag {
    /// Control tag value for an empty bucket.
    pub(crate) const EMPTY: Tag = Tag(0b1111_1111);

    /// Control tag value for a deleted bucket.
    pub(crate) const DELETED: Tag = Tag(0b1000_0000);

    /// Checks whether a control tag represents a full bucket (top bit is clear).
    #[inline]
    #[cfg_attr(kani, kani::ensures(|result| *result == (self.0 & 0x80 == 0)))]
    pub(crate) const fn is_full(self) -> bool {
        self.0 & 0x80 == 0
    }

    /// Checks whether a control tag represents a special value (top bit is set).
    #[inline]
    #[cfg_attr(kani, kani::ensures(|result| *result == (self.0 & 0x80 != 0)))]
    pub(crate) const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }

    /// Checks whether a special control value is EMPTY (just check 1 bit).
    #[inline]
    #[cfg_attr(kani, kani::requires(self.is_special()))]
    #[cfg_attr(kani, kani::ensures(|result| *result == (self.0 & 0x01 != 0)))]
    pub(crate) const fn special_is_empty(self) -> bool {
        debug_assert!(self.is_special());
        self.0 & 0x01 != 0
    }

    /// Creates a control tag representing a full bucket with the given hash.
    #[inline]
    #[cfg_attr(kani, kani::ensures(|result| result.is_full()))]
    #[cfg_attr(kani, kani::ensures(|result| result.0 & 0x80 == 0))]
    pub(crate) const fn full(hash: u64) -> Tag {
        // Constant for function that grabs the top 7 bits of the hash.
        const MIN_HASH_LEN: usize = if mem::size_of::<usize>() < mem::size_of::<u64>() {
            mem::size_of::<usize>()
        } else {
            mem::size_of::<u64>()
        };

        // Grab the top 7 bits of the hash. While the hash is normally a full 64-bit
        // value, some hash functions (such as FxHash) produce a usize result
        // instead, which means that the top 32 bits are 0 on 32-bit platforms.
        // So we use MIN_HASH_LEN constant to handle this.
        let top7 = hash >> (MIN_HASH_LEN * 8 - 7);
        Tag((top7 & 0x7f) as u8) // truncation
    }
}

#[cfg(kani)]
mod kani_proofs {
    use super::*;

    #[kani::proof_for_contract(Tag::is_full)]
    fn contract_tag_is_full() {
        let t = Tag(kani::any());
        t.is_full();
    }

    #[kani::proof_for_contract(Tag::is_special)]
    fn contract_tag_is_special() {
        let t = Tag(kani::any());
        t.is_special();
    }

    #[kani::proof_for_contract(Tag::special_is_empty)]
    fn contract_tag_special_is_empty() {
        let byte: u8 = kani::any();
        kani::assume(byte & 0x80 != 0);
        Tag(byte).special_is_empty();
    }

    #[kani::proof_for_contract(Tag::full)]
    fn contract_tag_full() {
        let hash: u8 = kani::any();
        Tag::full(hash as u64);
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_special() {
            if self.special_is_empty() {
                f.pad("EMPTY")
            } else {
                f.pad("DELETED")
            }
        } else {
            f.debug_tuple("full").field(&(self.0 & 0x7F)).finish()
        }
    }
}

/// Extension trait for slices of tags.
pub(crate) trait TagSliceExt {
    /// Fills the control with the given tag.
    fn fill_tag(&mut self, tag: Tag);

    /// Clears out the control.
    #[inline]
    fn fill_empty(&mut self) {
        self.fill_tag(Tag::EMPTY);
    }
}
impl TagSliceExt for [mem::MaybeUninit<Tag>] {
    #[inline]
    fn fill_tag(&mut self, tag: Tag) {
        // SAFETY: We have access to the entire slice, so, we can write to the entire slice.
        unsafe { self.as_mut_ptr().write_bytes(tag.0, self.len()) }
    }
}
