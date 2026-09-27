---
okf_version: "0.2"
type: Function
title: next_gaussian
description: "Generates standard normal sample via Box-Muller transform: N(0, 1)."
resource: crates/oxide-sim/src/analysis/monte_carlo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:03Z"
concept_id: crates/oxide-sim/src/analysis/monte_carlo/next_gaussian
language: rust
---

# next_gaussian

Generates standard normal sample via Box-Muller transform: N(0, 1).

## Signature

```rust
impl DeterministicPrng { pub fn next_gaussian(&mut self) -> f64 }
```

## Visibility

- `pub`

## Docstring

Generates standard normal sample via Box-Muller transform: N(0, 1).

## Source
Lines 61–65 in `crates/oxide-sim/src/analysis/monte_carlo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [monte_carlo](/crates/oxide-sim/src/analysis/monte_carlo.md) |
