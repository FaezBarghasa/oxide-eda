---
okf_version: "0.2"
type: Function
title: compute_line_search_damping
description: "Computes optimal line-search damping factor $\\lambda \\in (0, 1]$ to satisfy Armijo condition."
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade/compute_line_search_damping_1
language: rust
---

# compute_line_search_damping

Computes optimal line-search damping factor $\lambda \in (0, 1]$ to satisfy Armijo condition.

## Signature

```rust
pub fn compute_line_search_damping(
        &self,
        residual_norm_prev: f64,
        residual_norm_next: f64,
        proposed_lambda: f64,
    ) -> f64
```

## Visibility

- `pub`

## Docstring

Computes optimal line-search damping factor $\lambda \in (0, 1]$ to satisfy Armijo condition.

## Source
Lines 85–96 in `crates/oxide-sim/src/engine/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-sim/src/engine/cascade.md) |
