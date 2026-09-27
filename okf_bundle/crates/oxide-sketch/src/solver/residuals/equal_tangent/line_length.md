---
okf_version: "0.2"
type: Function
title: line_length
description: Resolve the length of a line from the current state vector.
resource: crates/oxide-sketch/src/solver/residuals/equal_tangent.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/solver/residuals/equal_tangent/line_length
language: rust
---

# line_length

Resolve the length of a line from the current state vector.

## Signature

```rust
fn line_length(
    line: SketchEntityId,
    state: &[f64],
    index: &EntityIndex,
    sketch: &SketchData,
) -> Result<f64, SketchError>
```

## Docstring

Resolve the length of a line from the current state vector.

## Source
Lines 28–38 in `crates/oxide-sketch/src/solver/residuals/equal_tangent.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [equal_tangent](/crates/oxide-sketch/src/solver/residuals/equal_tangent.md) |
| calls | [point_xy](/crates/oxide-sketch/src/solver/state/point_xy.md) |
| calls | [norm](/crates/oxide-sketch/src/solver/math/norm.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
| called_by | [equal_length](/crates/oxide-sketch/src/solver/residuals/equal_tangent/equal_length.md) |
