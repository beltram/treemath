#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, copath_bench);
criterion_main!(benches);
