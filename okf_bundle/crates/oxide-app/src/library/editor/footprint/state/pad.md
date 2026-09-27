---
okf_version: "0.2"
type: Module
title: pad
description: "Editor-side `Pad` mirror, pad-stack overrides, side enum, and"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad
language: rust
---

# pad

Editor-side `Pad` mirror, pad-stack overrides, side enum, and

## Docstring

Editor-side `Pad` mirror, pad-stack overrides, side enum, and
courtyard rect. Pure data types — no canvas state, no tool state.

## Relationships

| Type | Target |
|------|--------|
| related | [EditorPad](/crates/oxide-app/src/library/editor/footprint/state/pad/EditorPad.md) |
| related | [PadStackUi](/crates/oxide-app/src/library/editor/footprint/state/pad/PadStackUi.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/state/pad/default.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/state/pad/default.md) |
| related | [new_default](/crates/oxide-app/src/library/editor/footprint/state/pad/new_default.md) |
| related | [new_npt_hole](/crates/oxide-app/src/library/editor/footprint/state/pad/new_npt_hole.md) |
| related | [primary_layer](/crates/oxide-app/src/library/editor/footprint/state/pad/primary_layer.md) |
| related | [bbox_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/bbox_mm.md) |
| related | [rotate_delta_to_world_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_world_mm.md) |
| related | [rotate_delta_to_local_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_local_mm.md) |
| related | [local_to_world_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/local_to_world_mm.md) |
| related | [world_to_local_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/world_to_local_mm.md) |
| related | [rotated_corners_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotated_corners_mm.md) |
| related | [rotated_aabb_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotated_aabb_mm.md) |
| related | [mirror_about_own_vertical_axis](/crates/oxide-app/src/library/editor/footprint/state/pad/mirror_about_own_vertical_axis.md) |
| related | [contains_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/contains_mm.md) |
| related | [from_pad](/crates/oxide-app/src/library/editor/footprint/state/pad/from_pad.md) |
| related | [to_pad](/crates/oxide-app/src/library/editor/footprint/state/pad/to_pad.md) |
| related | [new_default](/crates/oxide-app/src/library/editor/footprint/state/pad/new_default.md) |
| related | [new_npt_hole](/crates/oxide-app/src/library/editor/footprint/state/pad/new_npt_hole.md) |
| related | [primary_layer](/crates/oxide-app/src/library/editor/footprint/state/pad/primary_layer.md) |
| related | [bbox_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/bbox_mm.md) |
| related | [rotate_delta_to_world_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_world_mm.md) |
| related | [rotate_delta_to_local_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotate_delta_to_local_mm.md) |
| related | [local_to_world_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/local_to_world_mm.md) |
| related | [world_to_local_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/world_to_local_mm.md) |
| related | [rotated_corners_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotated_corners_mm.md) |
| related | [rotated_aabb_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/rotated_aabb_mm.md) |
| related | [mirror_about_own_vertical_axis](/crates/oxide-app/src/library/editor/footprint/state/pad/mirror_about_own_vertical_axis.md) |
| related | [contains_mm](/crates/oxide-app/src/library/editor/footprint/state/pad/contains_mm.md) |
| related | [from_pad](/crates/oxide-app/src/library/editor/footprint/state/pad/from_pad.md) |
| related | [to_pad](/crates/oxide-app/src/library/editor/footprint/state/pad/to_pad.md) |
| related | [PadSide](/crates/oxide-app/src/library/editor/footprint/state/pad/PadSide.md) |
| related | [label](/crates/oxide-app/src/library/editor/footprint/state/pad/label.md) |
| related | [label](/crates/oxide-app/src/library/editor/footprint/state/pad/label.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/footprint/state/pad/fmt.md) |
| related | [fmt](/crates/oxide-app/src/library/editor/footprint/state/pad/fmt.md) |
| related | [from](/crates/oxide-app/src/library/editor/footprint/state/pad/from.md) |
| related | [from](/crates/oxide-app/src/library/editor/footprint/state/pad/from.md) |
| related | [from](/crates/oxide-app/src/library/editor/footprint/state/pad/from.md) |
| related | [from](/crates/oxide-app/src/library/editor/footprint/state/pad/from.md) |
| related | [NextPadDefaults](/crates/oxide-app/src/library/editor/footprint/state/pad/NextPadDefaults.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/state/pad/default.md) |
| related | [default](/crates/oxide-app/src/library/editor/footprint/state/pad/default.md) |
| related | [CourtyardRect](/crates/oxide-app/src/library/editor/footprint/state/pad/CourtyardRect.md) |
| related | [AlignOp](/crates/oxide-app/src/library/editor/footprint/state/pad/AlignOp.md) |
| related | [carry_links_by_unique_number](/crates/oxide-app/src/library/editor/footprint/state/pad/carry_links_by_unique_number.md) |
| related | [relink_pads_to_sketch](/crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch.md) |
| related | [pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pos_key.md) |
| related | [pad_pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pad_pos_key.md) |
| related | [resolve_link](/crates/oxide-app/src/library/editor/footprint/state/pad/resolve_link.md) |
