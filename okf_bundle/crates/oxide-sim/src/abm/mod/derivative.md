---
okf_version: "0.2"
type: Function
title: derivative
description: "Computes exact symbolic partial derivative with respect to node voltage `target_node`: df/d(v_target)."
resource: crates/oxide-sim/src/abm/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T12:29:23Z"
concept_id: crates/oxide-sim/src/abm/mod/derivative
language: rust
---

# derivative

Computes exact symbolic partial derivative with respect to node voltage `target_node`: df/d(v_target).

## Signature

```rust
impl AbmExpr { pub fn derivative(&self, target_node: usize) -> AbmExpr }
```

## Visibility

- `pub`

## Docstring

Computes exact symbolic partial derivative with respect to node voltage `target_node`: df/d(v_target).

## Source
Lines 144–176 in `crates/oxide-sim/src/abm/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [abm](/crates/oxide-sim/src/abm/mod.md) |
