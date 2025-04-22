/// Returns the height of that node in the tree
#[inline(always)]
pub const fn level(node_index: usize) -> usize {
    node_index.trailing_ones() as usize
}

/// Returns the root node
#[inline(always)]
pub const fn root(leaf_count: usize) -> usize {
    // leaf_count.wrapping_sub(1); // works only in case leaf_count is a power of 2
    if leaf_count == 0 {
        return 0;
    }
    let shl = node_width(leaf_count).ilog2();
    let pow2: usize = 1 << shl;
    pow2.wrapping_sub(1)
}

/// Number of nodes needed to represent a tree with [leaf_count] leaves.
#[inline(always)]
pub const fn node_width(leaf_count: usize) -> usize {
    if leaf_count == 0 {
        return 0;
    }
    // 2*(n - 1) + 1
    leaf_count
        .wrapping_sub(1) // since leaf_count is >= 1
        .saturating_mul(2)
        .saturating_add(1)
}

#[allow(dead_code)]
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
    pub const fn round_up_power_2(mut n: u32) -> u32 {
        n -= 1;
        n |= n.wrapping_shr(1);
        n |= n.wrapping_shr(2);
        n |= n.wrapping_shr(4);
        n |= n.wrapping_shr(8);
        n |= n.wrapping_shr(16);
        n += 1;
        n
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
}

#[cfg(test)]
mod tests {
    use super::{bounds::*, naive::*, *};

    mod level {
        use super::*;

        #[test]
        fn should_succeed() {
            for i in 0usize..100_000 {
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
            assert_eq!(node_width(usize::MAX), NODE_WIDTH_MAX);
        }
    }
}

pub mod bounds {
    pub const NODE_INDEX_MAX: usize = usize::MAX - 1;
    pub const LEAF_COUNT_MAX: usize = (NODE_INDEX_MAX / 2) + 1;
    pub const NODE_WIDTH_MAX: usize = (LEAF_COUNT_MAX - 1) * 2 + 1;
    pub const ROOT_MAX: usize = LEAF_COUNT_MAX - 1;

    pub const LEAF_COUNT_BITS: usize = 31;

    pub fn leaf_count_range() -> impl Iterator<Item = usize> {
        (0..=LEAF_COUNT_BITS).map(|sh| 1 << sh)
    }
}

#[cfg(any(test, feature = "bench"))]
pub mod naive {
    use super::*;

    #[inline(always)]
    pub fn level_naive(node_index: usize) -> usize {
        if node_index & 0x01 == 0 {
            return 0;
        }

        let mut k = 0;
        while node_index.checked_shr(k).is_some() && (node_index >> k) & 0x01 == 1 {
            k += 1;
        }
        k as usize
    }

    #[inline(always)]
    pub fn root_naive(leaf_count: usize) -> usize {
        if leaf_count == 0 {
            return 0;
        }
        let width = node_width(leaf_count);
        let pow2: usize = 1 << width.ilog2();
        pow2.wrapping_sub(1)
    }
}
