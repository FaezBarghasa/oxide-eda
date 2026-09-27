---
okf_version: "0.2"
type: Function
title: sidecar_point
description: "Position of the `Point` that a `shape_params` sidecar key names."
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/sidecar_point
language: rust
---

# sidecar_point

Position of the `Point` that a `shape_params` sidecar key names.

## Signature

```rust
fn sidecar_point(sketch: &SketchData, pad: &EditorPad, key: &str) -> (f64, f64)
```

## Docstring

Position of the `Point` that a `shape_params` sidecar key names.

## Source
Lines 132–153 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [SketchEntityId](/crates/oxide-sketch/src/id/SketchEntityId.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [properties_panel_rotation_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/properties_panel_rotation_regenerates_the_chamfer_anchor.md) |
| called_by | [resizing_a_chamfered_pad_regenerates_its_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/resizing_a_chamfered_pad_regenerates_its_chamfer_anchor.md) |
| called_by | [rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle](/crates/oxide-app/tests/footprint_pad_remint/rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle.md) |
| called_by | [sketch_centre_drag_carries_the_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/sketch_centre_drag_carries_the_chamfer_anchor_with_it.md) |
| called_by | [sketch_corner_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_corner_drag_regenerates_the_chamfer_anchor.md) |
| called_by | [sketch_edge_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor.md) |
| called_by | [translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it.md) |
