---
okf_version: "0.2"
type: Function
title: add_diag
description: "In-place diagonal add: `A[i][i] += λ` for all `i`. Used by LM to"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/add_diag
language: rust
---

# add_diag

In-place diagonal add: `A[i][i] += λ` for all `i`. Used by LM to

## Signature

```rust
pub fn add_diag(a: &mut [Vec<f64>], lambda: f64)
```

## Visibility

- `pub`

## Docstring

In-place diagonal add: `A[i][i] += λ` for all `i`. Used by LM to
damp the normal-equation matrix.

## Source
Lines 206–210 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [add_diag_basic](/crates/oxide-sketch/src/solver/math/add_diag_basic.md) |
