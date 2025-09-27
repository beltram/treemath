#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, sibling_bench);
criterion_main!(benches);
