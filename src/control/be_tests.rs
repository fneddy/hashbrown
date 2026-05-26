use super::{group::Group, Tag};
use std::vec::Vec;

#[test]
fn group_match_masks_follow_memory_order() {
    let width = Group::WIDTH;
    assert!(width >= 8);

    let filler = Tag::full(4 << 57);
    let mut tags = vec![filler; width];
    tags[0] = Tag::DELETED;
    tags[1] = Tag::full(0);
    tags[2] = Tag::EMPTY;
    tags[3] = Tag::full(1 << 57);
    tags[4] = Tag::DELETED;
    tags[5] = Tag::EMPTY;
    tags[6] = Tag::full(2 << 57);
    tags[width - 1] = Tag::full(3 << 57);

    let group = unsafe { Group::load(tags.as_ptr()) };

    let deleted_or_empty: Vec<_> = group.match_empty_or_deleted().into_iter().collect();
    assert_eq!(deleted_or_empty, vec![0, 2, 4, 5]);

    let empty: Vec<_> = group.match_empty().into_iter().collect();
    assert_eq!(empty, vec![2, 5]);

    let tag0: Vec<_> = group.match_tag(Tag::full(0)).into_iter().collect();
    assert_eq!(tag0, vec![1]);

    let tag1: Vec<_> = group.match_tag(Tag::full(1 << 57)).into_iter().collect();
    assert_eq!(tag1, vec![3]);

    let tag2: Vec<_> = group.match_tag(Tag::full(2 << 57)).into_iter().collect();
    assert_eq!(tag2, vec![6]);

    let tag3: Vec<_> = group.match_tag(Tag::full(3 << 57)).into_iter().collect();
    assert_eq!(tag3, vec![width - 1]);

    let filler_matches: Vec<_> = group.match_tag(filler).into_iter().collect();
    let mut expected_filler = Vec::new();
    for idx in 7..(width - 1) {
        expected_filler.push(idx);
    }
    assert_eq!(filler_matches, expected_filler);

    let full: Vec<_> = group.match_full().into_iter().collect();
    let mut expected_full = vec![1, 3, 6];
    expected_full.extend(expected_filler.iter().copied());
    expected_full.push(width - 1);
    assert_eq!(full, expected_full);
}
