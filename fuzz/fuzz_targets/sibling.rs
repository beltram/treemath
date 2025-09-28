#![no_main]

#[derive(Debug)]
struct Input {
    node_index_constrained: u32,
    leaf_count_constrained: u32,
    node_index_unconstrained: u32,
    leaf_count_unconstrained: u32,
}

impl arbitrary::Arbitrary<'_> for Input {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let leaf_count_constrained = u.int_in_range(0..=treemath::bounds::LEAF_COUNT_MAX)?;
        let node_index_constrained = leaf_count_constrained.saturating_sub(1).saturating_mul(2);
        let leaf_count_unconstrained = u32::arbitrary(u)?;
        let node_index_unconstrained = u32::arbitrary(u)?;
        Ok(Self { node_index_constrained, leaf_count_constrained, node_index_unconstrained, leaf_count_unconstrained })
    }
}

libfuzzer_sys::fuzz_target!(|input: Input| {
    treemath::sibling_unchecked(input.node_index_constrained, input.leaf_count_constrained);
    treemath::sibling(input.node_index_unconstrained, input.leaf_count_unconstrained);
});
