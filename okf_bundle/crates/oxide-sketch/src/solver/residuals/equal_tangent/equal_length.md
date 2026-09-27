---
okf_version: "0.2"
type: Function
title: equal_length
description: "EqualLength: `|d2| − |d1| = 0`."
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_length
language: rust
---

# equal_length

EqualLength: `|d2| − |d1| = 0`.

## Signature

```rust
pub fn equal_length(
    l1: SketchEntityId,
    l2: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<Vec<f64>, SketchError>
```

## Visibility

- `pub`

## Docstring

EqualLength: `|d2| − |d1| = 0`.

## Source
Lines 84–94 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [line_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length.md) |
| called_by | [residual](/crates/oxide-sketch/src/solver/residual/residual.md) |
