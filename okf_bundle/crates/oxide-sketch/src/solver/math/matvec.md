---
okf_version: "0.2"
type: Function
title: matvec
description: "`y = A · x` for an `m × n` matrix `A` (row-major, `Vec<Vec<f64>>`)."
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/matvec
language: rust
---

# matvec

`y = A · x` for an `m × n` matrix `A` (row-major, `Vec<Vec<f64>>`).

## Signature

```rust
pub fn matvec(a: &[Vec<f64>], x: &[f64]) -> Vec<f64>
```

## Visibility

- `pub`

## Docstring

`y = A · x` for an `m × n` matrix `A` (row-major, `Vec<Vec<f64>>`).
Returns a fresh `Vec<f64>` of length `m`. Panics if any row of
`A` has length ≠ `x.len()`.

## Source
Lines 139–151 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [matvec_2x3](/crates/oxide-sketch/src/solver/math/matvec_2x3.md) |
