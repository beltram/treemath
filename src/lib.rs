pub fn level(node_index: usize) -> usize {
    node_index.trailing_ones() as usize
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
pub mod tests {
    use super::naive::*;
    use super::*;

    pub mod level {
        use super::*;

        #[test]
        fn should_succeed() {
            for i in 0usize..100_000 {
                assert_eq!(level(i), level_naive(i), "failed for node index {}", i);
            }
        }

        #[test]
        fn should_succeed_at_boundaries() {
            let highmost_node_index = usize::MAX / 2;
            assert_eq!(level(highmost_node_index), level_naive(highmost_node_index));
            assert_eq!(level(0), level_naive(0));
        }
    }
}

#[cfg(any(test, feature = "bench"))]
pub mod naive {
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
}
