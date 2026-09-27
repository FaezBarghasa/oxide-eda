---
okf_version: "0.2"
type: Class
title: SolveResult
description: Output of a single solve. The state vector layout matches
resource: crates/oxide-sketch/src/solver/lm.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/lm/SolveResult
language: rust
---

# SolveResult

Output of a single solve. The state vector layout matches

## Signature

```rust
pub struct SolveResult
```

## Decorators

- `derive(Debug, Clone)`

## Visibility

- `pub`

## Docstring

Output of a single solve. The state vector layout matches
`pack(sketch).vector` — pair it with the [`EntityIndex`] returned
by `pack(...)` to recover individual entity coordinates.
[derive(Debug, Clone)]

## Methods

- `state`
- `index`
- `iterations`
- `final_residual_norm`
- `elapsed_ms`

## Source
Lines 66–72 in `crates/oxide-sketch/src/solver/lm.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lm](/crates/oxide-sketch/src/solver/lm.md) |
