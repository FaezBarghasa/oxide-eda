---
okf_version: "0.2"
type: Function
title: rotate_graphic_90
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_90
language: rust
---

# rotate_graphic_90

## Signature

```rust
fn rotate_graphic_90(
    sym: &mut Symbol,
    idx: usize,
    clockwise: bool,
    graphic_pivot_mode: GraphicRotationPivotMode,
)
```

## Source
Lines 65–153 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [graphic_geometry_center](/crates/oxide-app/src/library/editor/symbol/state/rotation/graphic_geometry_center.md) |
| calls | [rotate_graphic_point_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_point_90.md) |
| calls | [normalize_angle_rad](/crates/oxide-types/src/rotation2d/normalize_angle_rad.md) |
| called_by | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
