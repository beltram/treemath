# treemath

Arithmetics for perfect binary trees. I repeat, **ONLY** for perfect binary trees with a flat in-order representation, otherwise it simply does not work.
Perfect binary trees are a subcategory of binary trees where each node has two children and all leaves are on the same level.

This crates tries to be as fast as optimal, going branchless wherever possible and using some bit twiddling techniques.

See [benchmarks](./BENCHMARKS.md) for an overview of the benefits over a naive implementation.
