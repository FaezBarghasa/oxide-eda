---
okf_version: "0.2"
type: Function
title: mint_pad_corner_outline
description: "v0.16 — mint 4 corner Points + 4 Lines outlining a pad's bbox."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_pad_corner_outline
language: rust
---

# mint_pad_corner_outline

v0.16 — mint 4 corner Points + 4 Lines outlining a pad's bbox.

## Signature

```rust
pub(super) fn mint_pad_corner_outline(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    pad: &EditorPad,
) -> [SketchEntityId; 4]
```

## Visibility

- `pub(super)`

## Docstring

v0.16 — mint 4 corner Points + 4 Lines outlining a pad's bbox.
Returns the corner IDs in `[ne, se, sw, nw]` order so the caller
can store them on `EditorPad.corner_entity_ids` and reposition
them on later pad moves. Both the corner Points and the Lines
connecting them are flagged `construction = true` so
`oxide_bake::bake_pads` skips them and they don't double up the
rendered pad geometry.

## Source
Lines 40–65 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.md) |
| calls | [push_construction_point](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_construction_point.md) |
| calls | [push_construction_line](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_construction_line.md) |
| called_by | [mint_chamfered_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_chamfered_pad_geometry.md) |
| called_by | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| called_by | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
| called_by | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
