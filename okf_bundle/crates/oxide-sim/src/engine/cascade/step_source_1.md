---
okf_version: "0.2"
type: Function
title: step_source
description: "Steps source scaling parameter $\\alpha \\in [0.0 \\to 1.0]$."
resource: crates/oxide-sim/src/engine/cascade.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:26:44Z"
concept_id: crates/oxide-sim/src/engine/cascade/step_source_1
language: rust
---

# step_source

Steps source scaling parameter $\alpha \in [0.0 \to 1.0]$.

## Signature

```rust
pub fn step_source(&mut self, success: bool) -> (f64, bool)
```

## Visibility

- `pub`

## Docstring

Steps source scaling parameter $\alpha \in [0.0 \to 1.0]$.

## Source
Lines 108–118 in `crates/oxide-sim/src/engine/cascade.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [cascade](/crates/oxide-sim/src/engine/cascade.md) |
