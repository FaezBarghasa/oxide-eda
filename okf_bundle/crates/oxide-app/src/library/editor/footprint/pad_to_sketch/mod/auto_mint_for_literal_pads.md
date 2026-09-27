---
okf_version: "0.2"
type: Function
title: auto_mint_for_literal_pads
description: When the user transitions into Sketch mode for the first time on
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/auto_mint_for_literal_pads
language: rust
---

# auto_mint_for_literal_pads

When the user transitions into Sketch mode for the first time on

## Signature

```rust
pub fn auto_mint_for_literal_pads(pads: &mut [EditorPad], footprint: &mut Footprint) -> usize
```

## Visibility

- `pub`

## Docstring

When the user transitions into Sketch mode for the first time on
a footprint that has literal pads but an empty sketch, mint a
`Point` + `PadAttr` for each pad. Writes the minted sketch entity
IDs back into each `EditorPad.sketch_entity_id` so subsequent
Pads-mode edits can mirror through the link. Returns the number
of entities minted (zero if the sketch already had content or no
literal pads existed).

## Source
Lines 82–120 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [sketch_is_authored](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/sketch_is_authored.md) |
| calls | [ensure_board_top_plane](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/ensure_board_top_plane.md) |
| calls | [pad_attr_from_editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/pad_attr_from_editor_pad.md) |
| calls | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
| called_by | [empty_pads_mint_nothing](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/empty_pads_mint_nothing.md) |
| called_by | [skip_when_sketch_already_has_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/skip_when_sketch_already_has_entities.md) |
| called_by | [skip_when_sketch_only_has_construction_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/skip_when_sketch_only_has_construction_entities.md) |
| called_by | [three_pads_mint_three_points_with_pad_attrs](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/three_pads_mint_three_points_with_pad_attrs.md) |
| called_by | [set_mode](/crates/oxide-app/src/library/editor/footprint/updates/view/set_mode.md) |
