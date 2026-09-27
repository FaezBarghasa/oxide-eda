---
okf_version: "0.2"
type: Function
title: rotate_graphic_point_90
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_point_90
language: rust
---

# rotate_graphic_point_90

## Signature

```rust
fn rotate_graphic_point_90(
    p: [f64; 2],
    geometry_center_world: [f64; 2],
    clockwise: bool,
    space: RotationSpace,
    pivot: RotationPivot,
) -> [f64; 2]
```

## Source
Lines 166–182 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [rotate_object](/crates/oxide-types/src/rotation2d/rotate_object.md) |
| called_by | [rotate_graphic_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_90.md) |
