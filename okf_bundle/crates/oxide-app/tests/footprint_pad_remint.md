---
okf_version: "0.2"
type: Module
title: footprint_pad_remint
description: "The invariant: exactly ONE owner of per-shape sidecar layout."
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint
language: rust
---

# footprint_pad_remint

The invariant: exactly ONE owner of per-shape sidecar layout.

## Docstring

The invariant: exactly ONE owner of per-shape sidecar layout.

A transform that changes the pad FRAME (rotate, flip) regenerates
the pad's sketch geometry through the mint path, rather than
through a second copy of the layout rules. Per-shape layout
knowledge lived in three places — the mint functions, the
post-solve reverse mirror, and the bbox-corner-only move mirror —
and that duplication is why the outline kept drifting out of step
with the copper.

These tests are written against a PARAMETRIC shape on purpose. The
bbox-corner mover is correct for `Rect` by construction, so
`Rect`-only coverage cannot see the defect at all: for `Rect` the
outline IS the four bbox corners, while a Chamfered / RoundRect /
Oval pad also owns anchors and arc centres that the corner mover
never touches.

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| related | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| related | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| related | [geometry_fingerprint](/crates/oxide-app/tests/footprint_pad_remint/geometry_fingerprint.md) |
| related | [centre_point](/crates/oxide-app/tests/footprint_pad_remint/centre_point.md) |
| related | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
| related | [rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle](/crates/oxide-app/tests/footprint_pad_remint/rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle.md) |
| related | [flip_keeps_the_baked_shape_equal_to_the_editor_shape](/crates/oxide-app/tests/footprint_pad_remint/flip_keeps_the_baked_shape_equal_to_the_editor_shape.md) |
| related | [one_undo_after_a_rotate_restores_the_prior_sketch_geometry](/crates/oxide-app/tests/footprint_pad_remint/one_undo_after_a_rotate_restores_the_prior_sketch_geometry.md) |
| related | [properties_panel_rotation_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/properties_panel_rotation_regenerates_the_chamfer_anchor.md) |
| related | [resizing_a_chamfered_pad_regenerates_its_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/resizing_a_chamfered_pad_regenerates_its_chamfer_anchor.md) |
| related | [translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/translating_a_chamfered_pad_carries_its_chamfer_anchor_with_it.md) |
| related | [horizontal_edge_at_y](/crates/oxide-app/tests/footprint_pad_remint/horizontal_edge_at_y.md) |
| related | [sketch_edge_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_edge_drag_regenerates_the_chamfer_anchor.md) |
| related | [a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick](/crates/oxide-app/tests/footprint_pad_remint/a_streamed_sketch_edge_drag_keeps_resizing_after_the_first_tick.md) |
| related | [sketch_corner_drag_regenerates_the_chamfer_anchor](/crates/oxide-app/tests/footprint_pad_remint/sketch_corner_drag_regenerates_the_chamfer_anchor.md) |
| related | [sketch_centre_drag_carries_the_chamfer_anchor_with_it](/crates/oxide-app/tests/footprint_pad_remint/sketch_centre_drag_carries_the_chamfer_anchor_with_it.md) |
