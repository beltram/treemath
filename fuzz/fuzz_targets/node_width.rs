#![no_main]

libfuzzer_sys::fuzz_target!(|leaf_count: u32| {
    treemath::node_width(leaf_count);
});
