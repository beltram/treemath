#![no_main]

#[derive(Debug)]
struct Input {
    self_node_index_constrained: u32,
    other_node_index_constrained: u32,
    self_node_index_unconstrained: u32,
    other_node_index_unconstrained: u32,
}

impl arbitrary::Arbitrary<'_> for Input {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let self_node_index_constrained = u.int_in_range(0..=treemath::bounds::NODE_INDEX_MAX)?;
        let self_node_index_unconstrained = u32::arbitrary(u)?;
        let other_node_index_constrained = u.int_in_range(0..=treemath::bounds::NODE_INDEX_MAX)?;
        let other_node_index_unconstrained = u32::arbitrary(u)?;
        Ok(Self { self_node_index_constrained, other_node_index_constrained, self_node_index_unconstrained, other_node_index_unconstrained })
    }
}

libfuzzer_sys::fuzz_target!(|input: Input| {
    treemath::common_ancestor_unchecked(input.self_node_index_constrained, input.other_node_index_constrained);
    treemath::common_ancestor(input.self_node_index_unconstrained, input.other_node_index_unconstrained);
});
