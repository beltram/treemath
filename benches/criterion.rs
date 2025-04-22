use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use treemath::naive::*;
use treemath::*;

const ITER: usize = 3;

fn level_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Level");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |node_index| black_box(level(node_index)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |node_index| black_box(level_naive(node_index)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

mod ranges {
    use itertools::Itertools;

    pub fn node_index(iter: usize) -> impl Iterator<Item = usize> {
        const MAX: usize = usize::MAX - 1;
        let iter = (0..=MAX).step_by(MAX / iter);
        iter.chain(std::iter::once(MAX)).dedup()
    }
}

/*fn root_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Root");
    group.plot_config(plot_config);
    let step = LeafCount::range().count().div_euclid(ITERATIONS as usize);
    for lc in LeafCount::range().step_by(step) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(
                || LeafCount(1 << rand::thread_rng().gen_range(0..=lc.ilog2())),
                |lc| black_box(lc.root()),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(
                || LeafCount(1 << rand::thread_rng().gen_range(0..=lc.ilog2())),
                |lc| black_box(lc.root_rfc9420()),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn parent_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parent");
    for i in NodeIndex::range(ITERATIONS) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.parent(LeafCount::MAX)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.parent_rfc9420(LeafCount::MAX)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn sibling_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sibling");
    for i in NodeIndex::range(ITERATIONS) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.sibling(LeafCount::MAX)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.sibling_rfc9420(LeafCount::MAX)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn left_right_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Left");
    for i in NodeIndex::range(ITERATIONS) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.left()),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.left_rfc9420()),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
    let mut group = c.benchmark_group("Right");
    for i in NodeIndex::range(ITERATIONS) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.right()),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || NodeIndex(rand::thread_rng().gen_range(0..=i.0)),
                |idx| black_box(idx.right_rfc9420()),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn direct_path_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Direct path");
    group.plot_config(plot_config);

    for lc in LeafCount::range().step_by(LeafCount::BITS / ITERATIONS as usize) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(
                || {
                    NodeIndex(rand::thread_rng().gen_range(0..=lc.0));
                    let idx = rand::thread_rng().gen_range(0u32..(lc.0 * 2) - 1);
                    (NodeIndex(idx), lc)
                },
                |(idx, lc)| black_box(idx.direct_path(lc)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("new-fast", lc), &lc, |b, &lc| {
            b.iter_batched(
                || {
                    NodeIndex(rand::thread_rng().gen_range(0..=lc.0));
                    let idx = rand::thread_rng().gen_range(0u32..(lc.0 * 2) - 1);
                    (NodeIndex(idx), lc)
                },
                |(idx, lc)| black_box(idx.direct_path_fast(lc)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(
                || {
                    NodeIndex(rand::thread_rng().gen_range(0..=lc.0));
                    let idx = rand::thread_rng().gen_range(0u32..(lc.0 * 2) - 1);
                    (NodeIndex(idx), lc)
                },
                |(idx, lc)| black_box(idx.direct_path_rfc9420(lc)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn copath_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Copath");
    group.plot_config(plot_config);

    for lc in LeafCount::range().step_by(LeafCount::BITS / ITERATIONS as usize) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(
                || {
                    NodeIndex(rand::thread_rng().gen_range(0..=lc.0));
                    let idx = rand::thread_rng().gen_range(0u32..(lc.0 * 2) - 1);
                    (NodeIndex(idx), lc)
                },
                |(idx, lc)| black_box(idx.copath(lc)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(
                || {
                    NodeIndex(rand::thread_rng().gen_range(0..=lc.0));
                    let idx = rand::thread_rng().gen_range(0u32..(lc.0 * 2) - 1);
                    (NodeIndex(idx), lc)
                },
                |(idx, lc)| black_box(idx.copath_rfc9420(lc)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn common_ancestor_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Common ancestor");
    for i in NodeIndex::range(ITERATIONS) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || {
                    let mut a = rand::thread_rng().gen_range(0..=i.0);
                    a &= !1;
                    let mut b = rand::thread_rng().gen_range(0..=i.0);
                    b &= !1;
                    (NodeIndex(a), NodeIndex(b))
                },
                |(a, b)| black_box(a.common_ancestor(b)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || {
                    let mut a = rand::thread_rng().gen_range(0..=i.0);
                    a &= !1;
                    let mut b = rand::thread_rng().gen_range(0..=i.0);
                    b &= !1;
                    (NodeIndex(a), NodeIndex(b))
                },
                |(a, b)| black_box(a.common_ancestor_rfc9420(b)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}*/

criterion_group!(
    benches,
    level_bench,
    // root_bench,
    // parent_bench,
    // sibling_bench,
    // left_right_bench,
    // direct_path_bench,
    // copath_bench,
    // common_ancestor_bench
);
criterion_main!(benches);
