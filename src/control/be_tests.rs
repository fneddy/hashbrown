use super::{group::Group, Tag};
use std::vec::Vec;

#[test]
fn group_match_masks_follow_memory_order() {
    let width = Group::WIDTH;
    assert!(width >= 8);

    let tag0 = Tag(0x00);
    let tag1 = Tag(0x11);
    let tag2 = Tag(0x22);
    let tag3 = Tag(0x33);
    let filler = Tag(0x44);

    let mut tags = vec![filler; width];
    tags[0] = Tag::DELETED;
    tags[1] = tag0;
    tags[2] = Tag::EMPTY;
    tags[3] = tag1;
    tags[4] = Tag::DELETED;
    tags[5] = Tag::EMPTY;
    tags[6] = tag2;
    tags[width - 1] = tag3;

    assert_ne!(tag0, tag1);
    assert_ne!(tag0, tag2);
    assert_ne!(tag0, tag3);
    assert_ne!(tag0, filler);
    assert_ne!(tag1, tag2);
    assert_ne!(tag1, tag3);
    assert_ne!(tag1, filler);
    assert_ne!(tag2, tag3);
    assert_ne!(tag2, filler);
    assert_ne!(tag3, filler);

    let group = unsafe { Group::load(tags.as_ptr()) };

    let deleted_or_empty: Vec<_> = group.match_empty_or_deleted().into_iter().collect();
    assert_eq!(deleted_or_empty, vec![0, 2, 4, 5]);

    let empty: Vec<_> = group.match_empty().into_iter().collect();
    assert_eq!(empty, vec![2, 5]);

    let tag0_matches: Vec<_> = group.match_tag(tag0).into_iter().collect();
    assert_eq!(tag0_matches, vec![1]);

    let tag1_matches: Vec<_> = group.match_tag(tag1).into_iter().collect();
    assert_eq!(tag1_matches, vec![3]);

    let tag2_matches: Vec<_> = group.match_tag(tag2).into_iter().collect();
    assert_eq!(tag2_matches, vec![6]);

    let tag3_matches: Vec<_> = group.match_tag(tag3).into_iter().collect();
    assert_eq!(tag3_matches, vec![width - 1]);

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
