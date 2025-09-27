#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, level_bench);
criterion_main!(benches);
