---
okf_version: "0.2"
type: Class
title: Solver
description: Top-level solver façade. Configure timeouts / iteration cap /
resource: crates/oxide-sketch/src/solver/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/mod/Solver
language: rust
---

# Solver

Top-level solver façade. Configure timeouts / iteration cap /

## Signature

```rust
pub struct Solver
```

## Decorators

- `derive(Clone, Debug)`

## Visibility

- `pub`

## Docstring

Top-level solver façade. Configure timeouts / iteration cap /
tolerance, then call [`Solver::solve`] to run an LM iteration
followed by DOF analysis in a single shot.

Public API patterned after the spec in
`docs/internal/SKETCH_MODE_v0.13_PLAN.md` Task 3.6.
[derive(Clone, Debug)]

## Methods

- `timeout_ms`
- `max_iters`
- `tolerance`

## Source
Lines 28–35 in `crates/oxide-sketch/src/solver/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [solver](/crates/oxide-sketch/src/solver/mod.md) |
