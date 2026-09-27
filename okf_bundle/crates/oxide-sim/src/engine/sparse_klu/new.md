---
okf_version: "0.2"
type: Function
title: new
description: "Creates an empty sparse matrix of dimension `nrows x ncols`."
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/new
language: rust
---

# new

Creates an empty sparse matrix of dimension `nrows x ncols`.

## Signature

```rust
impl SparseMatrixCsc { pub fn new(nrows: usize, ncols: usize) -> Self }
```

## Visibility

- `pub`

## Docstring

Creates an empty sparse matrix of dimension `nrows x ncols`.

## Source
Lines 23–31 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
