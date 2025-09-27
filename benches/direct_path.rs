#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, direct_path_bench);
criterion_main!(benches);
