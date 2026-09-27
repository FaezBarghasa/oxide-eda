---
okf_version: "0.2"
type: Function
title: from_triplets
description: "Constructs CSC matrix from coordinate list (COO) triples (row, col, value)."
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/from_triplets
language: rust
---

# from_triplets

Constructs CSC matrix from coordinate list (COO) triples (row, col, value).

## Signature

```rust
impl SparseMatrixCsc { pub fn from_triplets(nrows: usize, ncols: usize, triplets: &[(usize, usize, f64)]) -> Self }
```

## Visibility

- `pub`

## Docstring

Constructs CSC matrix from coordinate list (COO) triples (row, col, value).

## Source
Lines 34–67 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
