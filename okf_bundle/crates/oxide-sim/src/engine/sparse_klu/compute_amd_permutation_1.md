---
okf_version: "0.2"
type: Function
title: compute_amd_permutation
description: Computes Approximate Minimum Degree (AMD) heuristic permutation vector P.
resource: crates/oxide-sim/src/engine/sparse_klu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:25:03Z"
concept_id: crates/oxide-sim/src/engine/sparse_klu/compute_amd_permutation_1
language: rust
---

# compute_amd_permutation

Computes Approximate Minimum Degree (AMD) heuristic permutation vector P.

## Signature

```rust
pub fn compute_amd_permutation(&self) -> Vec<usize>
```

## Visibility

- `pub`

## Docstring

Computes Approximate Minimum Degree (AMD) heuristic permutation vector P.

## Source
Lines 94–104 in `crates/oxide-sim/src/engine/sparse_klu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sparse_klu](/crates/oxide-sim/src/engine/sparse_klu.md) |
