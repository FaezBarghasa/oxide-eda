---
okf_version: "0.2"
type: Function
title: axpy
description: "In-place AXPY: `y[i] += α · x[i]` for all `i`. Panics on length"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/axpy
language: rust
---

# axpy

In-place AXPY: `y[i] += α · x[i]` for all `i`. Panics on length

## Signature

```rust
pub fn axpy(alpha: f64, x: &[f64], y: &mut [f64])
```

## Visibility

- `pub`

## Docstring

In-place AXPY: `y[i] += α · x[i]` for all `i`. Panics on length
mismatch.

## Source
Lines 129–134 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [axpy_in_place](/crates/oxide-sketch/src/solver/math/axpy_in_place.md) |
| called_by | [axpy_length_mismatch_panics](/crates/oxide-sketch/src/solver/math/axpy_length_mismatch_panics.md) |
