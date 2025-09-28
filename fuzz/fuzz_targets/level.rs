#![no_main]

libfuzzer_sys::fuzz_target!(|node_index: u32| {
    treemath::level(node_index);
});
