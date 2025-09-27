#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, left_right_bench);
criterion_main!(benches);
