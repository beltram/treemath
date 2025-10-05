#![no_main]

#[derive(Debug)]
struct Input {
    node_index: u32,
    leaf_count: u32,
}

impl arbitrary::Arbitrary<'_> for Input {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> arbitrary::Result<Self> {
        let leaf_count = u32::arbitrary(u)?;
        let node_index = u32::arbitrary(u)?;
        Ok(Self { node_index, leaf_count })
    }
}

libfuzzer_sys::fuzz_target!(|input: Input| {
    treemath::copath(input.node_index, input.leaf_count);
});
