---
okf_version: "0.2"
type: Function
title: norm_sq_2
description: "Squared Euclidean norm `|v|² = v.x² + v.y²`. Avoids the `sqrt`"
resource: crates/oxide-sketch/src/solver/math.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/math/norm_sq_2
language: rust
---

# norm_sq_2

Squared Euclidean norm `|v|² = v.x² + v.y²`. Avoids the `sqrt`

## Signature

```rust
pub fn norm_sq_2(v: Vec2) -> f64
```

## Decorators

- `inline`

## Visibility

- `pub`

## Docstring

Squared Euclidean norm `|v|² = v.x² + v.y²`. Avoids the `sqrt`
when only relative magnitudes matter (e.g. distance comparisons,
LM convergence test on `|r|²`).
[inline]

## Source
Lines 80–82 in `crates/oxide-sketch/src/solver/math.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [math](/crates/oxide-sketch/src/solver/math.md) |
| called_by | [symmetric_about_line](/crates/oxide-sketch/src/solver/residuals/symmetric_midpoint/symmetric_about_line.md) |
