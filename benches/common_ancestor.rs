#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, common_ancestor_bench);
criterion_main!(benches);
