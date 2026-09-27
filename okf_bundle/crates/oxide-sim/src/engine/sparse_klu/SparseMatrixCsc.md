---
okf_version: "0.2"
type: Class
title: SparseMatrixCsc
description: Compressed Sparse Column (CSC) representation for circuit MNA matrices.
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/SparseMatrixCsc
language: rust
---

# SparseMatrixCsc

Compressed Sparse Column (CSC) representation for circuit MNA matrices.

## Signature

```rust
pub struct SparseMatrixCsc
```

## Decorators

- `derive(Debug, Clone, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Compressed Sparse Column (CSC) representation for circuit MNA matrices.
[derive(Debug, Clone, Serialize, Deserialize)]

## Methods

- `nrows`
- `ncols`
- `col_ptrs`
- `row_indices`
- `values`

## Source
Lines 13–19 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
