#![no_main]

libfuzzer_sys::fuzz_target!(|leaf_count: u32| {
    treemath::root(leaf_count);
});
