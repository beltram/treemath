use criterion::{AxisScale, BatchSize, BenchmarkId, Criterion, PlotConfiguration, criterion_group, criterion_main};
use itertools::Itertools;
use std::hint::black_box;
use treemath::bounds::*;
use treemath::naive::*;
use treemath::*;

const ITER: u32 = 3;

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
    use super::*;

    pub fn node_index(step: u32) -> impl Iterator<Item = u32> {
        let iter = (0..=NODE_INDEX_MAX).step_by(NODE_INDEX_MAX as usize / step as usize);
        iter.chain(std::iter::once(NODE_INDEX_MAX)).dedup()
    }
}

fn root_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Root");
    group.plot_config(plot_config);
    let step = leaf_count_range().count().div_euclid(ITER as usize);
    for lc in leaf_count_range().step_by(step) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(|| 1 << rand::random_range(0..=lc.ilog2()), |lc| black_box(root(lc)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(|| 1 << rand::random_range(0..=lc.ilog2()), |lc| black_box(root_naive(lc)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn parent_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parent");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(parent(idx, LEAF_COUNT_MAX)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(parent_naive(idx, LEAF_COUNT_MAX)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn sibling_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sibling");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(sibling(idx, LEAF_COUNT_MAX)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(sibling_naive(idx, LEAF_COUNT_MAX)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn left_right_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Left");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(left(idx)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(left_naive(idx)), BatchSize::SmallInput)
        });
    }
    group.finish();
    let mut group = c.benchmark_group("Right");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(right(idx)), BatchSize::SmallInput)
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(|| rand::random_range(0..=i), |idx| black_box(right_naive(idx)), BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn direct_path_bench(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Direct path");
    group.plot_config(plot_config);

    for lc in leaf_count_range().step_by(LEAF_COUNT_BITS as usize / ITER as usize) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(
                || (rand::random_range(0u32..(lc * 2) - 1), lc),
                |(idx, lc)| black_box(direct_path(idx, lc)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(
                || (rand::random_range(0u32..(lc * 2) - 1), lc),
                |(idx, lc)| black_box(direct_path_naive(idx, lc)),
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

    for lc in leaf_count_range().step_by(LEAF_COUNT_BITS as usize / ITER as usize) {
        group.bench_with_input(BenchmarkId::new("new", lc), &lc, |b, &lc| {
            b.iter_batched(
                || (rand::random_range(0u32..(lc * 2) - 1), lc),
                |(idx, lc)| black_box(copath(idx, lc)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", lc), &lc, |b, &lc| {
            b.iter_batched(
                || (rand::random_range(0u32..(lc * 2) - 1), lc),
                |(idx, lc)| black_box(copath_naive(idx, lc)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

fn common_ancestor_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Common ancestor");
    for i in ranges::node_index(ITER) {
        group.bench_with_input(BenchmarkId::new("new", i), &i, |b, &i| {
            b.iter_batched(
                || {
                    let mut a = rand::random_range(0..=i);
                    a &= !1;
                    let mut b = rand::random_range(0..=i);
                    b &= !1;
                    (a, b)
                },
                |(a, b)| black_box(common_ancestor(a, b)),
                BatchSize::SmallInput,
            )
        });
        group.bench_with_input(BenchmarkId::new("naive", i), &i, |b, &i| {
            b.iter_batched(
                || {
                    let mut a = rand::random_range(0..=i);
                    a &= !1;
                    let mut b = rand::random_range(0..=i);
                    b &= !1;
                    (a, b)
                },
                |(a, b)| black_box(common_ancestor_naive(a, b)),
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    level_bench,
    root_bench,
    parent_bench,
    sibling_bench,
    left_right_bench,
    direct_path_bench,
    copath_bench,
    common_ancestor_bench
);
criterion_main!(benches);
