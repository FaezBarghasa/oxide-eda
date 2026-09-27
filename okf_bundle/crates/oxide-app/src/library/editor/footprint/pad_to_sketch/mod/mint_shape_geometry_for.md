---
okf_version: "0.2"
type: Function
title: mint_shape_geometry_for
description: "Branch on `pad.shape` to mint the correct sketch geometry — Circle"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for
language: rust
---

# mint_shape_geometry_for

Branch on `pad.shape` to mint the correct sketch geometry — Circle

## Signature

```rust
fn mint_shape_geometry_for(
    sketch: &mut SketchData,
    plane_id: oxide_sketch::plane::PlaneId,
    pad: &mut EditorPad,
    entity_id: SketchEntityId,
)
```

## Docstring

Branch on `pad.shape` to mint the correct sketch geometry — Circle
for Round, parametric anchors+arcs for RoundRect, stadium for Oval,
chamfer outline for Chamfered, plain bbox-corner outline for Rect /
Custom / etc. Writes `pad.corner_entity_ids` accordingly.

Shared by `mirror_add_pad_to_sketch` (Pads-mode click → mint) and
`auto_mint_for_literal_pads` (first Sketch-mode entry on a footprint
authored before sketch existed). Without sharing, legacy pads
always got the bbox outline regardless of shape — Round pads in
particular ended up with 4 disconnected corner Points, so dragging
one corner just deformed the outline without resizing the pad.

## Source
Lines 224–276 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [mint_round_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_pad_geometry.md) |
| calls | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
| calls | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| calls | [mint_chamfered_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_chamfered_pad_geometry.md) |
| calls | [mint_pad_corner_outline](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_pad_corner_outline.md) |
| calls | [record_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger.md) |
| called_by | [auto_mint_for_literal_pads](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads.md) |
| called_by | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
