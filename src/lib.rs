use crate::bounds::LEVEL_MAX;
use std::collections::VecDeque;

/// Returns the height of that node in the tree
#[inline(always)]
pub const fn level(node_index: u32) -> u32 {
    node_index.trailing_ones()
}

/// Returns the root node
#[inline(always)]
pub const fn root(leaf_count: u32) -> u32 {
    // leaf_count.wrapping_sub(1); // works only in case leaf_count is a power of 2
    if leaf_count == 0 {
        return 0;
    }
    let shl = node_width(leaf_count).ilog2();
    let pow2: u32 = 1 << shl;
    pow2.wrapping_sub(1)
}

/// Number of nodes needed to represent a tree with [leaf_count] leaves.
#[inline(always)]
pub const fn node_width(leaf_count: u32) -> u32 {
    if leaf_count == 0 {
        return 0;
    }
    // 2*(n - 1) + 1
    leaf_count
        .wrapping_sub(1) // since leaf_count is >= 1
        .saturating_mul(2)
        .saturating_add(1)
}

/// Get the parent of a node, return [None] if the node is the root
#[inline(always)]
pub const fn parent(node_index: u32, leaf_count: u32) -> Option<u32> {
    if node_index == root(leaf_count) {
        return None;
    }
    Some((bits::last_zero_bit(node_index) | node_index) & !bits::last_zero_bit(node_index).wrapping_shl(1))
}

/// Given a node, return the left/right child of his parent, return [None] when root
#[inline(always)]
pub const fn sibling(node_index: u32, leaf_count: u32) -> Option<u32> {
    let Some(parent) = parent(node_index, leaf_count) else {
        return None;
    };
    let parent = parent as isize;
    let d = parent.overflowing_sub(node_index as isize).0;
    let sibling = parent.overflowing_add(d).0;
    Some(sibling as u32)
}

#[inline(always)]
pub const fn left(node_index: u32) -> Option<u32> {
    if node_index % 2 == 0 {
        return None;
    }
    let lzb = bits::last_zero_bit(node_index);
    let left = node_index & !lzb.wrapping_shr(1);
    Some(left)
}

#[inline(always)]
pub const fn right(node_index: u32) -> Option<u32> {
    if node_index % 2 == 0 {
        return None;
    }
    let lzb = bits::last_zero_bit(node_index);
    let right = (node_index | lzb) & !lzb.wrapping_shr(1);
    Some(right)
}

/// Returns (left, right)
#[inline(always)]
pub const fn children(node_index: u32) -> Option<(u32, u32)> {
    if node_index % 2 == 0 {
        return None;
    }
    let lzb = bits::last_zero_bit(node_index);
    let mask = !lzb.wrapping_shr(1);

    let left = node_index & mask;
    let right = (node_index | lzb) & mask;

    Some((left, right))
}

pub fn direct_path(node_index: u32, leaf_count: u32) -> Option<VecDeque<u32>> {
    // see https://mmapped.blog/posts/22-flat-in-order-trees.html#sec-addressing
    let mut root = root(leaf_count);
    if node_index == root {
        return None;
    }

    let mut root_level = level(root);

    let floor = LEVEL_MAX.wrapping_sub(root_level);
    let node_level = level(node_index);
    let mut path_size = root_level.wrapping_sub(node_level);
    let mut path = VecDeque::with_capacity(path_size as usize);

    path.push_back(root);

    path_size = path_size.wrapping_sub(1);

    let mask = u32::MAX >> floor;
    let chunk = node_index & mask;

    for _ in 0..path_size {
        let d = ((chunk >> root_level) & 1) == 0;
        root = child_with_direction(root, d, root_level);
        root_level = root_level.wrapping_sub(1);
        path.push_front(root);
    }

    Some(path)
}

#[inline(always)]
pub const fn child_with_direction(node_index: u32, direction: bool, level: u32) -> u32 {
    let f = 2u32 ^ (1u32.wrapping_shl(direction as u32) | 1);
    let lvl = level.wrapping_sub(1);
    let f = f.wrapping_shl(lvl);
    node_index ^ f
}

#[inline(always)]
const fn nephew(node_index: u32, is_left: bool, leaf_count: u32, mut level: u32) -> Option<u32> {
    if level < 1 {
        return None;
    }
    let Some(parent) = parent(node_index, leaf_count) else {
        return None;
    };
    level = level.wrapping_add(1);
    let sibling = child_with_direction(parent, node_index > parent, level);
    level = level.wrapping_sub(1);
    let nephew = child_with_direction(sibling, is_left, level);
    Some(nephew)
}

pub fn copath(node_index: u32, leaf_count: u32) -> Option<VecDeque<u32>> {
    // see https://mmapped.blog/posts/22-flat-in-order-trees.html#sec-addressing
    let mut root = root(leaf_count);
    if node_index == root {
        return None;
    }

    let mut root_level = level(root);

    let floor = LEVEL_MAX.wrapping_sub(root_level);
    let node_level = level(node_index);
    let path_size = root_level.wrapping_sub(node_level).wrapping_sub(1);
    let mut copath = VecDeque::with_capacity(path_size as usize);

    let mask = u32::MAX >> floor;
    let chunk = node_index & mask;

    let b = ((chunk >> root_level) & 1) == 0;
    root = child_with_direction(root, !b, root_level);
    root_level = root_level.wrapping_sub(1);
    copath.push_front(root);

    for _ in 0..path_size {
        let b = ((chunk >> root_level) & 1) == 0;

        let Some(nephew) = nephew(root, !b, leaf_count, root_level) else {
            // should never happen because we should bail before the leaf if we compute the path_size right
            return Some(copath);
        };

        root = nephew;
        root_level = root_level.wrapping_sub(1);
        copath.push_front(root);
    }

    Some(copath)
}

#[inline(always)]
pub const fn common_ancestor(node_index: u32, other: u32) -> u32 {
    if node_index == other {
        return node_index;
    }
    let d = bits::most_significant_bit(node_index ^ other);
    (node_index & !d) | (d.wrapping_sub(1))
}

mod bits {
    #[inline(always)]
    pub const fn last_set_bit(n: u32) -> u32 {
        n.wrapping_sub(n.wrapping_sub(1) & n)
    }

    #[inline(always)]
    pub const fn last_zero_bit(n: u32) -> u32 {
        last_set_bit(n + 1)
    }

    #[inline(always)]
    pub const fn most_significant_bit(mut n: u32) -> u32 {
        n |= n.wrapping_shr(1);
        n |= n.wrapping_shr(2);
        n |= n.wrapping_shr(4);
        n |= n.wrapping_shr(8);
        n |= n.wrapping_shr(16);
        n - n.wrapping_shr(1)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn round_up_power_2(mut n: u32) -> u32 {
        n -= 1;
        n |= n.wrapping_shr(1);
        n |= n.wrapping_shr(2);
        n |= n.wrapping_shr(4);
        n |= n.wrapping_shr(8);
        n |= n.wrapping_shr(16);
        n |= n.wrapping_shr(32);
        n += 1;
        n
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn msb_should_succeed() {
            assert_eq!(1, most_significant_bit(1));
            assert_eq!(2, most_significant_bit(2));
            assert_eq!(2, most_significant_bit(3));
            assert_eq!(4, most_significant_bit(4));
            assert_eq!(4, most_significant_bit(5));
            assert_eq!(4, most_significant_bit(6));
            assert_eq!(1 << 31, most_significant_bit(u32::MAX));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{bounds::*, naive::*, *};
    use itertools::*;

    mod level {
        use super::*;

        #[test]
        fn should_succeed() {
            for i in 0u32..100_000 {
                assert_eq!(level(i), level_naive(i), "failed for node index {}", i);
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            assert_eq!(level(NODE_INDEX_MAX), level_naive(NODE_INDEX_MAX));
            assert_eq!(level(0), level_naive(0));
        }
    }

    mod root {
        use super::*;

        #[test]
        fn should_succeed() {
            for lc in leaf_count_range() {
                assert_eq!(root(lc), root_naive(lc));
                assert_eq!(root(lc), lc - 1);
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            assert_eq!(root(0), root_naive(0));
            assert_eq!(root(0), 0);
            assert_eq!(root(LEAF_COUNT_MAX), root_naive(LEAF_COUNT_MAX));
            assert_eq!(root(LEAF_COUNT_MAX), ROOT_MAX);
        }
    }

    mod node_width {
        use super::*;

        #[test]
        fn should_succeed() {
            assert_eq!(node_width(1), 1);
            assert_eq!(node_width(2), 3);
        }

        #[test]
        fn should_succeed_at_boundaries() {
            assert_eq!(node_width(0), 0);
            assert_eq!(node_width(LEAF_COUNT_MAX), NODE_WIDTH_MAX);
            assert_eq!(node_width(u32::MAX), NODE_WIDTH_MAX);
        }
    }

    mod parent {
        use super::*;

        #[test]
        fn should_fail_when_root() {
            for (r, lc) in root_range() {
                assert!(parent(r, lc).is_none());
                assert!(parent_naive(r, lc).is_none());
            }
        }

        #[test]
        fn should_succeed_for_leaves() {
            let lc = LEAF_COUNT_MAX;
            for left_leaf in (0..=u16::MAX).step_by(4) {
                assert_eq!(parent(left_leaf as u32, lc), parent_naive(left_leaf as u32, lc));
                assert_eq!(parent(left_leaf as u32, lc), Some(left_leaf as u32 + 1));
            }
            for right_leaf in (2..=u16::MAX).step_by(4) {
                assert_eq!(parent(right_leaf as u32, lc), parent_naive(right_leaf as u32, lc));
                assert_eq!(parent(right_leaf as u32, lc), Some(right_leaf as u32 - 1));
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            assert!(parent(NODE_INDEX_MAX, LEAF_COUNT_MAX).is_some());
            assert_eq!(parent(NODE_INDEX_MAX, LEAF_COUNT_MAX), parent_naive(NODE_INDEX_MAX, LEAF_COUNT_MAX));
        }
    }

    mod direct_path {
        use super::*;

        #[test]
        fn should_succeed() {
            for (lc, i) in leaf_count_range_with_node_index().take(100_000) {
                assert_eq!(direct_path(i, lc), direct_path_naive(i, lc));
            }
        }

        #[test]
        fn should_succeed_for_remarkable_values() {
            let lc = 8u32;
            let values = [
                (0u32, vec![1u32, 3, 7]),
                (1, vec![3, 7]),
                (2, vec![1, 3, 7]),
                (3, vec![7]),
                (4, vec![5, 3, 7]),
                (5, vec![3, 7]),
                (6, vec![5, 3, 7]),
                (8, vec![9, 11, 7]),
                (9, vec![11, 7]),
                (10, vec![9, 11, 7]),
                (11, vec![7]),
                (12, vec![13, 11, 7]),
                (13, vec![11, 7]),
                (14, vec![13, 11, 7]),
            ]
            .map(|(i, e)| (i, VecDeque::from_iter(e)));
            for (i, expected) in values {
                assert_eq!(direct_path(i, lc), direct_path_naive(i, lc));
                assert_eq!(direct_path(i, lc), Some(expected));
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            let values = [
                (0, 1),
                (0, 2),
                (1, 2),
                (NODE_INDEX_MAX - 2, LEAF_COUNT_MAX),
                (NODE_INDEX_MAX - 1, LEAF_COUNT_MAX),
                (NODE_INDEX_MAX, LEAF_COUNT_MAX),
            ];
            for (i, lc) in values {
                assert_eq!(direct_path(i, lc), direct_path_naive(i, lc));
            }
        }

        #[test]
        fn should_fail_for_roots() {
            for (r, lc) in root_range() {
                assert!(direct_path(r, lc).is_none());
                assert_eq!(direct_path(r, lc), direct_path_naive(r, lc));
            }
        }
    }

    mod copath {
        use super::*;

        #[test]
        fn should_succeed() {
            for (lc, i) in leaf_count_range_with_node_index().take(10) {
                assert_eq!(copath(i, lc), copath_naive(i, lc));
            }
        }

        #[test]
        fn should_succeed_for_remarkable_values() {
            let lc = 8u32;
            let values = [
                (0u32, vec![2u32, 5, 11]),
                (1, vec![5, 11]),
                (2, vec![0, 5, 11]),
                (3, vec![11]),
                (4, vec![6, 1, 11]),
                (5, vec![1, 11]),
                (6, vec![4, 1, 11]),
                (8, vec![10, 13, 3]),
                (9, vec![13, 3]),
                (10, vec![8, 13, 3]),
                (11, vec![3]),
                (12, vec![14, 9, 3]),
                (13, vec![9, 3]),
                (14, vec![12, 9, 3]),
            ]
            .map(|(i, e)| (i, VecDeque::from_iter(e)));
            for (i, expected) in values {
                assert_eq!(copath(i, lc), copath_naive(i, lc));
                assert_eq!(copath(i, lc), Some(expected));
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            let values = [
                (0, 1),
                (0, 2),
                (1, 2),
                (NODE_INDEX_MAX - 2, LEAF_COUNT_MAX),
                (NODE_INDEX_MAX - 1, LEAF_COUNT_MAX),
                (NODE_INDEX_MAX, LEAF_COUNT_MAX),
            ];
            for (i, lc) in values {
                assert_eq!(copath(i, lc), copath_naive(i, lc));
            }
        }

        #[test]
        fn should_fail_for_roots() {
            for (r, lc) in root_range() {
                assert!(copath(r, lc).is_none());
                assert_eq!(copath(r, lc), copath_naive(r, lc));
            }
        }
    }

    mod common_ancestor {
        use super::*;

        #[test]
        fn should_succeed() {
            let e = 10;
            for a in level_range(0).take(1 << e) {
                for b in level_range(0).take(1 << e) {
                    assert_eq!(common_ancestor(a, b), common_ancestor_naive(a, b));
                }
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            let values = [(0, 2), (0, NODE_INDEX_MAX)];
            for (a, b) in values {
                assert_eq!(common_ancestor(a, b), common_ancestor_naive(a, b));
            }
        }

        fn level_range(level: u32) -> impl Iterator<Item = u32> {
            let lower = (1 << level) - 1;
            let step = 1 << (level + 1);
            (lower..=NODE_INDEX_MAX).step_by(step).dedup()
        }
    }
}

pub mod bounds {
    pub const NODE_INDEX_MAX: u32 = u32::MAX - 1;
    pub const LEAF_COUNT_MAX: u32 = (NODE_INDEX_MAX / 2) + 1;
    pub const NODE_WIDTH_MAX: u32 = (LEAF_COUNT_MAX - 1) * 2 + 1;
    pub const ROOT_MAX: u32 = LEAF_COUNT_MAX - 1;
    pub const LEVEL_MAX: u32 = u32::BITS - 1;

    pub const LEAF_COUNT_BITS: u32 = 31;
    pub const ROOT_BITS: u32 = 31;

    pub fn leaf_count_range() -> impl Iterator<Item = u32> {
        (0..=LEAF_COUNT_BITS).map(|sh| 1 << sh)
    }

    // returns an iterator of (root, leaf_count)
    pub fn root_range() -> impl Iterator<Item = (u32, u32)> {
        (0..=ROOT_BITS).map(|e| (1 << e) - 1).map(|root| (root, root + 1))
    }

    pub fn leaf_count_range_with_node_index() -> impl Iterator<Item = (u32, u32)> {
        leaf_count_range().flat_map(|lc| (0..=lc.saturating_sub(1) * 2).map(move |i| (lc, i)))
    }
}

#[cfg(any(test, feature = "bench"))]
pub mod naive {
    use super::*;
    use std::collections::VecDeque;
    use std::ops::Shl;

    #[inline(always)]
    pub fn level_naive(node_index: u32) -> u32 {
        if node_index & 0x01 == 0 {
            return 0;
        }

        let mut k = 0;
        while node_index.checked_shr(k).is_some() && (node_index >> k) & 0x01 == 1 {
            k += 1;
        }
        k as u32
    }

    #[inline(always)]
    pub fn root_naive(leaf_count: u32) -> u32 {
        if leaf_count == 0 {
            return 0;
        }
        let width = node_width(leaf_count);
        let pow2: u32 = 1 << width.ilog2();
        pow2.wrapping_sub(1)
    }

    #[inline(always)]
    pub fn parent_naive(node_index: u32, leaf_count: u32) -> Option<u32> {
        if node_index == root_naive(leaf_count) {
            return None;
        }

        let k = level_naive(node_index);
        let b = (node_index >> (k + 1)) & 0x01;
        Some((node_index | (1 << k)) ^ (b << (k + 1)))
    }

    #[inline(always)]
    pub fn sibling_naive(node_index: u32, leaf_count: u32) -> Option<u32> {
        let parent = parent_naive(node_index, leaf_count)?;
        if node_index < parent { right_naive(parent) } else { left_naive(parent) }
    }

    #[inline(always)]
    pub fn left_naive(node_index: u32) -> Option<u32> {
        let k = level_naive(node_index);
        if k == 0 {
            return None;
        }
        let node_index = node_index ^ (0x01 << k.wrapping_sub(1));
        Some(node_index)
    }

    #[inline(always)]
    pub fn right_naive(node_index: u32) -> Option<u32> {
        let k = level_naive(node_index);
        if k == 0 {
            return None;
        }
        let node_index = node_index ^ (0x03 << k.wrapping_sub(1));
        Some(node_index)
    }

    #[inline(always)]
    pub fn direct_path_naive(mut node_index: u32, leaf_count: u32) -> Option<VecDeque<u32>> {
        let root = root_naive(leaf_count);
        if node_index == root {
            return None;
        }

        let mut ret = VecDeque::new();
        while node_index != root {
            match parent_naive(node_index, leaf_count) {
                Some(parent_idx) => node_index = parent_idx,
                None => return None,
            }
            ret.push_back(node_index);
        }

        Some(ret)
    }

    pub fn copath_naive(node_index: u32, leaf_count: u32) -> Option<VecDeque<u32>> {
        if node_index == root(leaf_count) {
            return None;
        }

        let mut path = direct_path(node_index, leaf_count)?;
        path.insert(0, node_index);
        let _ = path.pop_back();

        path.into_iter().map(|path_idx| sibling(path_idx, leaf_count)).collect()
    }

    pub fn common_ancestor_naive(mut node_index: u32, mut other: u32) -> u32 {
        let self_lvl = level_naive(node_index).saturating_add(1);
        let other_lvl = level_naive(other).saturating_add(1);
        if self_lvl <= other_lvl && (node_index >> other_lvl) == (other >> other_lvl) {
            return other;
        } else if other_lvl <= self_lvl && (node_index >> self_lvl) == (other >> self_lvl) {
            return node_index;
        }

        let mut k = 0u32;
        while node_index != other {
            node_index >>= 1;
            other >>= 1;
            k = k.saturating_add(1);
        }

        let s = 1u32.shl(k.saturating_sub(1));
        (node_index.overflowing_shl(k).0).saturating_add(s).saturating_sub(1)
    }
}
