---
okf_version: "0.2"
type: Module
title: sparse_klu
description: "Sparse Matrix Kernel with Approximate Minimum Degree (AMD) & Block Triangular Form (BTF)."
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu
language: rust
---

# sparse_klu

Sparse Matrix Kernel with Approximate Minimum Degree (AMD) & Block Triangular Form (BTF).

## Docstring

Sparse Matrix Kernel with Approximate Minimum Degree (AMD) & Block Triangular Form (BTF).

Conforms to Master Technical Directive Horizon I (§2, Task 1.1):
- Compressed Sparse Column (CSC) representation optimized for unsymmetric circuit matrices.
- Approximate Minimum Degree (AMD) pre-ordering to minimize fill-in.
- Block Triangular Form (BTF) decomposition isolating irreducible diagonal submatrices.
- Markowitz threshold partial pivoting during LU factorization.

## Relationships

| Type | Target |
|------|--------|
| related | [SparseMatrixCsc](/crates/oxide-sim/src/engine/sparse_klu/SparseMatrixCsc.md) |
| related | [new](/crates/oxide-sim/src/engine/sparse_klu/new.md) |
| related | [from_triplets](/crates/oxide-sim/src/engine/sparse_klu/from_triplets.md) |
| related | [nnz](/crates/oxide-sim/src/engine/sparse_klu/nnz.md) |
| related | [multiply_vector](/crates/oxide-sim/src/engine/sparse_klu/multiply_vector.md) |
| related | [compute_amd_permutation](/crates/oxide-sim/src/engine/sparse_klu/compute_amd_permutation.md) |
| related | [compute_btf_blocks](/crates/oxide-sim/src/engine/sparse_klu/compute_btf_blocks.md) |
| related | [solve_dense_equivalent](/crates/oxide-sim/src/engine/sparse_klu/solve_dense_equivalent.md) |
| related | [new](/crates/oxide-sim/src/engine/sparse_klu/new.md) |
| related | [from_triplets](/crates/oxide-sim/src/engine/sparse_klu/from_triplets.md) |
| related | [nnz](/crates/oxide-sim/src/engine/sparse_klu/nnz.md) |
| related | [multiply_vector](/crates/oxide-sim/src/engine/sparse_klu/multiply_vector.md) |
| related | [compute_amd_permutation](/crates/oxide-sim/src/engine/sparse_klu/compute_amd_permutation.md) |
| related | [compute_btf_blocks](/crates/oxide-sim/src/engine/sparse_klu/compute_btf_blocks.md) |
| related | [solve_dense_equivalent](/crates/oxide-sim/src/engine/sparse_klu/solve_dense_equivalent.md) |
| related | [test_sparse_csc_triplets_and_multiply](/crates/oxide-sim/src/engine/sparse_klu/test_sparse_csc_triplets_and_multiply.md) |
| related | [test_sparse_linear_solve](/crates/oxide-sim/src/engine/sparse_klu/test_sparse_linear_solve.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
