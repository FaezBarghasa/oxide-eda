---
okf_version: "0.2"
type: Function
title: mint_pad_entities
description: "Mint a pad's centre `Point` + `PadAttr` + per-shape sidecar geometry"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities
language: rust
---

# mint_pad_entities

Mint a pad's centre `Point` + `PadAttr` + per-shape sidecar geometry

## Signature

```rust
fn mint_pad_entities(pad: &mut EditorPad, footprint: &mut Footprint, entity_id: SketchEntityId)
```

## Docstring

Mint a pad's centre `Point` + `PadAttr` + per-shape sidecar geometry
under `entity_id`. The single body behind both the first mint
(`mirror_add_pad_to_sketch`) and the re-mint
(`remint_pad_geometry`), so a transform can never regenerate
geometry through a second, differently-behaved copy of the layout
rules.

## Source
Lines 146–161 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [ensure_board_top_plane](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/ensure_board_top_plane.md) |
| calls | [pad_attr_from_editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/pad_attr_from_editor_pad.md) |
| calls | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
| called_by | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
