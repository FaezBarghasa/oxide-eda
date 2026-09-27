---
okf_version: "0.2"
type: Module
title: pad_to_sketch
description: Mint sketch entities for literal pads — bidirectional sketch ↔
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod
language: rust
---

# pad_to_sketch

Mint sketch entities for literal pads — bidirectional sketch ↔

## Docstring

Mint sketch entities for literal pads — bidirectional sketch ↔
pads sync.

When the user enters Sketch mode for a footprint that has literal
pads but no sketch entities yet, this module auto-creates a
`Point` + `PadAttr` for every pad so the round-trip is identity-
preserving.

Submodules:
- [`helpers`] — small `push_point` / `push_line` / `push_arc_ccw`
primitives that collapse the repeated mint blocks.
- [`attr`] — `EditorPad ↔ PadAttr` mapping and the BoardTop plane
helper.
- [`mint`] — per-shape `mint_*_pad_geometry` functions.
- [`remint_in_place`] — the id-preserving re-mint a live drag needs.
- [`solve`] — post-solve "reverse mirror" helpers.
- [`ownership`] — the single answer to "which sketch entities does
this pad own?", shared by the move and delete mirrors.

## Relationships

| Type | Target |
|------|--------|
| related | [rotation_expr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/rotation_expr.md) |
| related | [sketch_is_authored](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sketch_is_authored.md) |
| related | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| related | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| related | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
| related | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| related | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
| related | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| related | [reassert_bbox_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/reassert_bbox_corners.md) |
| related | [sidecar_id](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sidecar_id.md) |
| related | [warn_profile_pad_untransformed](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/warn_profile_pad_untransformed.md) |
| related | [is_sketch_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/is_sketch_profile_pad.md) |
| related | [point_xy_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/point_xy_of.md) |
| related | [profile_seed_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/profile_seed_line.md) |
| related | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
| related | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
