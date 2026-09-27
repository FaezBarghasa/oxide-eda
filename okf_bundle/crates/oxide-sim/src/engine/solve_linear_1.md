---
okf_version: "0.2"
type: Function
title: solve_linear
description: "Solves linear system A * x = b via Gaussian elimination with partial pivoting."
resource: crates/oxide-sim/src/engine.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:33:35Z"
concept_id: crates/oxide-sim/src/engine/solve_linear_1
language: rust
---

# solve_linear

Solves linear system A * x = b via Gaussian elimination with partial pivoting.

## Signature

```rust
fn solve_linear(&self, a: &[f64], b: &[f64]) -> Result<Vec<f64>, SimError>
```

## Docstring

Solves linear system A * x = b via Gaussian elimination with partial pivoting.

## Source
Lines 128–195 in `crates/oxide-sim/src/engine.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [engine](/crates/oxide-sim/src/engine.md) |
