---
okf_version: "0.2"
type: Module
title: bench_linalg
description: Self-contained benchmark for the in-house dense LU solver in
resource: crates/oxide-sketch/examples/bench_linalg.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-sketch/examples/bench_linalg
language: rust
---

# bench_linalg

Self-contained benchmark for the in-house dense LU solver in

## Docstring

Self-contained benchmark for the in-house dense LU solver in
`oxide_sketch::solver::linalg`. No external benchmarking crate
is used — timings are taken with `std::time::Instant` so the
Apache-clean Oxide codebase stays free of dev-dependencies for
micro-benchmarking.

Run:

```sh
cargo run -p oxide-sketch --example bench_linalg --release
```

What it measures
----------------
For a small spread of system sizes (10, 25, 50, 100, 200 unknowns)
we time:

1. `lu_decompose` — the partial-pivot LU factorisation (the
dominant cost; O(n³)).
2. `lu_solve`     — forward + back substitution given an existing
LU + permutation (O(n²)).
3. `solve`        — end-to-end `solve(A, b)` (decompose + solve
every call; the LM iteration's amortised cost per step).

Each size is benchmarked over `iters` runs of the same problem
(regenerated once, reused) to amortise allocator and timer noise.
The reported number is the per-iteration mean.

What we expect
--------------
For `n ≤ 200` unknowns (the v0.13 sketcher's worst-case constraint
count), end-to-end solve should be sub-millisecond on a 2024-class
laptop. The LM iteration's 50 ms timeout comfortably covers a full
Newton–Marquardt loop (50–100 iterations × sub-ms solve each).

What we do NOT claim
--------------------
These numbers are not vs-state-of-the-art benchmarks. nalgebra and
faer (the Apache-2.0/MIT pure-Rust LA libraries) ship SIMD-tuned
dense LU that will be 2–5× faster on these sizes. We choose
roll-our-own to keep the Apache-clean Oxide codebase
dependency-free; the bench exists to verify the performance is
adequate for the v0.13 use case, not to compete with hand-tuned
BLAS implementations.

## Relationships

| Type | Target |
|------|--------|
| related | [make_matrix](/crates/oxide-sketch/examples/bench_linalg/make_matrix.md) |
| related | [bench](/crates/oxide-sketch/examples/bench_linalg/bench.md) |
| related | [main](/crates/oxide-sketch/examples/bench_linalg/main.md) |
