---
okf_version: "0.2"
type: Function
title: multiply_vector
description: "Multiplies sparse matrix by dense vector: y = A * x."
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/multiply_vector
language: rust
---

# multiply_vector

Multiplies sparse matrix by dense vector: y = A * x.

## Signature

```rust
impl SparseMatrixCsc { pub fn multiply_vector(&self, x: &[f64], y: &mut [f64]) }
```

## Visibility

- `pub`

## Docstring

Multiplies sparse matrix by dense vector: y = A * x.

## Source
Lines 75–91 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
