---
okf_version: "0.2"
type: Function
title: compute_btf_blocks
description: Block Triangular Form (BTF) partition analysis.
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/compute_btf_blocks
language: rust
---

# compute_btf_blocks

Block Triangular Form (BTF) partition analysis.

## Signature

```rust
impl SparseMatrixCsc { pub fn compute_btf_blocks(&self) -> Vec<usize> }
```

## Visibility

- `pub`

## Docstring

Block Triangular Form (BTF) partition analysis.
Returns diagonal block boundaries [0, b1, b2, ..., ncols].

## Source
Lines 108–111 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
