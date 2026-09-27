---
okf_version: "0.2"
type: Function
title: rotate_selected_about_geometry_center
description: Convenience wrapper for geometry-center graphic rotation scenarios.
resource: crates/oxide-app/src/library/editor/symbol/state/rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_about_geometry_center
language: rust
---

# rotate_selected_about_geometry_center

Convenience wrapper for geometry-center graphic rotation scenarios.

## Signature

```rust
pub fn rotate_selected_about_geometry_center(
    sym: &mut Symbol,
    sel: Option<SymbolSelection>,
    clockwise: bool,
)
```

## Visibility

- `pub`

## Docstring

Convenience wrapper for geometry-center graphic rotation scenarios.

## Source
Lines 52–63 in `crates/oxide-app/src/library/editor/symbol/state/rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rotation](/crates/oxide-app/src/library/editor/symbol/state/rotation.md) |
| calls | [rotate_selected_with_pivot](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected_with_pivot.md) |
| called_by | [rotate_selected_about_geometry_center_keeps_text_anchor_fixed](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_keeps_text_anchor_fixed.md) |
| called_by | [rotate_selected_about_geometry_center_rotates_polygon_vertices](/crates/oxide-app/src/library/editor/symbol/state/tests/rotate_selected_about_geometry_center_rotates_polygon_vertices.md) |
