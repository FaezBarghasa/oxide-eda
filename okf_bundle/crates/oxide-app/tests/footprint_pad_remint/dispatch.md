---
okf_version: "0.2"
type: Function
title: dispatch
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/dispatch
language: rust
---

# dispatch

## Signature

```rust
fn dispatch(app: &mut Oxide, path: &Path, msg: FootprintEditorMsg)
```

## Source
Lines 33–38 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| called_by | [a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick](/crates/oxide-app/tests/footprint_pad_remint/a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick.md) |
| called_by | [flip_keeps_the_baked_shape_equal_to_the_editor_shape](/crates/oxide-app/tests/footprint_pad_remint/flip_keeps_the_baked_shape_equal_to_the_editor_shape.md) |
| called_by | [one_undo_after_a_rotate_restores_the_prior_sketch_geometry](/crates/oxide-app/tests/footprint_pad_remint/one_undo_after_a_rotate_restores_the_prior_sketch_geometry.md) |
| called_by | [rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle](/crates/oxide-app/tests/footprint_pad_remint/rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle.md) |
| called_by | [sketch_centre_drag_carries_the_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/sketch_centre_drag_carries_the_chamfer_anchor_with_it.md) |
| called_by | [sketch_corner_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_corner_drag_regenerates_the_chamfer_anchor.md) |
| called_by | [sketch_edge_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor.md) |
| called_by | [translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it.md) |
