#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, parent_bench);
criterion_main!(benches);
