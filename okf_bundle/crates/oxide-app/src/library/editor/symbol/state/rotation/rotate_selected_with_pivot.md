---
okf_version: "0.2"
type: Function
title: rotate_selected_with_pivot
description: Rotate the selected entity by 90° with explicit graphic pivot mode.
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot
language: rust
---

# rotate_selected_with_pivot

Rotate the selected entity by 90° with explicit graphic pivot mode.

## Signature

```rust
pub fn rotate_selected_with_pivot(
    sym: &mut Symbol,
    sel: Option<SymbolSelection>,
    clockwise: bool,
    graphic_pivot_mode: GraphicRotationPivotMode,
)
```

## Visibility

- `pub`

## Docstring

Rotate the selected entity by 90° with explicit graphic pivot mode.

## Source
Lines 16–49 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [pin_body_delta](/crates/oxide-app/src/library/editor/symbol/state/rotation/pin_body_delta.md) |
| calls | [rotate_vec](/crates/oxide-types/src/anchor2d/rotate_vec.md) |
| calls | [snap_axis_value](/crates/oxide-app/src/library/editor/symbol/state/rotation/snap_axis_value.md) |
| calls | [rotate_pin_orientation_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_pin_orientation_90.md) |
| calls | [rotate_graphic_90](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_graphic_90.md) |
| called_by | [rotate_selected](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected.md) |
| called_by | [rotate_selected_about_geometry_center](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_about_geometry_center.md) |
| called_by | [rotate_selected_about_geometry_center_keeps_rectangle_center](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_rectangle_center.md) |
| called_by | [apply_symbol_transform](/crates/oxide-app/src/library/editor/symbol/updates/transform/apply_symbol_transform.md) |
