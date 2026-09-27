---
okf_version: "0.2"
type: Function
title: matvec_t
description: "`y = Aᵀ · x` for an `m × n` matrix `A` (row-major). Returns a"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/matvec_t
language: rust
---

# matvec_t

`y = Aᵀ · x` for an `m × n` matrix `A` (row-major). Returns a

## Signature

```rust
pub fn matvec_t(a: &[Vec<f64>], x: &[f64]) -> Vec<f64>
```

## Visibility

- `pub`

## Docstring

`y = Aᵀ · x` for an `m × n` matrix `A` (row-major). Returns a
fresh `Vec<f64>` of length `n`. Panics if `x.len() != m` or any
row width differs.

## Source
Lines 156–172 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [matvec_t_3x2](/crates/oxide-sketch/src/solver/math/matvec_t_3x2.md) |
