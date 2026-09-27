---
okf_version: "0.2"
type: Function
title: generate_runs
description: Generates $N$ reproducible parameter variation sets using a fixed seed.
resource: crates/oxide-sim/src/analysis/monte_carlo.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:30:03Z"
concept_id: crates/oxide-sim/src/analysis/monte_carlo/generate_runs_1
language: rust
---

# generate_runs

Generates $N$ reproducible parameter variation sets using a fixed seed.

## Signature

```rust
pub fn generate_runs(
        parameters: &[TolerancedParameter],
        num_runs: usize,
        seed: u64,
    ) -> Vec<MonteCarloRun>
```

## Visibility

- `pub`

## Docstring

Generates $N$ reproducible parameter variation sets using a fixed seed.

## Source
Lines 73–108 in `crates/oxide-sim/src/analysis/monte_carlo.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [monte_carlo](/crates/oxide-sim/src/analysis/monte_carlo.md) |
