#[path = "all.rs"]
mod all;

use all::*;

criterion_group!(benches, root_bench);
criterion_main!(benches);
