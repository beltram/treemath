#![no_main]

#[derive(Debug)]
struct Input {
    node_index_constrained: u32,
    node_index_unconstrained: u32,
}

impl arbitrary::Arbitrary<'_> for Input {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let node_index_constrained = u.int_in_range(0..=treemath::bounds::NODE_INDEX_MAX)?;
        let node_index_unconstrained = u32::arbitrary(u)?;
        Ok(Self { node_index_constrained, node_index_unconstrained })
    }
}

libfuzzer_sys::fuzz_target!(|input: Input| {
    treemath::left_unchecked(input.node_index_constrained);
    treemath::right_unchecked(input.node_index_constrained);
    treemath::left(input.node_index_unconstrained);
    treemath::right(input.node_index_unconstrained);
});
