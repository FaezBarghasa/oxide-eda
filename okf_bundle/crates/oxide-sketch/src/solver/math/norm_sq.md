---
okf_version: "0.2"
type: Function
title: norm_sq
description: "`|x|²` for a flat vector. The LM convergence test compares this"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/norm_sq
language: rust
---

# norm_sq

`|x|²` for a flat vector. The LM convergence test compares this

## Signature

```rust
pub fn norm_sq(x: &[f64]) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

`|x|²` for a flat vector. The LM convergence test compares this
to a tolerance.
[inline]

## Source
Lines 117–119 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [solve_lm](/crates/oxide-sketch/src/solver/lm/solve_lm.md) |
| called_by | [norm_vec](/crates/oxide-sketch/src/solver/math/norm_vec.md) |
