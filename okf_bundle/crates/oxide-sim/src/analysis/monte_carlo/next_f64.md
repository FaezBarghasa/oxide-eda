---
okf_version: "0.2"
type: Function
title: next_f64
description: "Generates uniform float in [0.0, 1.0)."
resource: crates/oxide-sim/src/analysis/monte_carlo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:03Z"
concept_id: crates/oxide-sim/src/analysis/monte_carlo/next_f64
language: rust
---

# next_f64

Generates uniform float in [0.0, 1.0).

## Signature

```rust
impl DeterministicPrng { pub fn next_f64(&mut self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Generates uniform float in [0.0, 1.0).

## Source
Lines 56–58 in `crates/oxide-sim/src/analysis/monte_carlo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [monte_carlo](/crates/oxide-sim/src/analysis/monte_carlo.md) |
