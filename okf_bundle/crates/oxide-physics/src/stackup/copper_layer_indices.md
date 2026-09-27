---
okf_version: "0.2"
type: Function
title: copper_layer_indices
description: Returns the indices of all conductive copper layers in order.
resource: crates/oxide-physics/src/stackup.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-physics"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-physics/src/stackup/copper_layer_indices
language: rust
---

# copper_layer_indices

Returns the indices of all conductive copper layers in order.

## Signature

```rust
impl LayerStackup { pub fn copper_layer_indices(&self) -> Vec<usize> }
```

## Visibility

- `pub`

## Docstring

Returns the indices of all conductive copper layers in order.

## Source
Lines 186–198 in `crates/oxide-physics/src/stackup.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stackup](/crates/oxide-physics/src/stackup.md) |
