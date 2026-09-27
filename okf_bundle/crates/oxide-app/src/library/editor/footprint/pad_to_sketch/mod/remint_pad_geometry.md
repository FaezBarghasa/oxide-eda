---
okf_version: "0.2"
type: Function
title: remint_pad_geometry
description: "Regenerate a pad's sketch sidecar after a transform that changes"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry
language: rust
---

# remint_pad_geometry

Regenerate a pad's sketch sidecar after a transform that changes

## Signature

```rust
pub fn remint_pad_geometry(pad: &mut EditorPad, footprint: &mut Footprint) -> bool
```

## Visibility

- `pub`

## Docstring

Regenerate a pad's sketch sidecar after a transform that changes
the pad FRAME — rotate, flip, anything that moves
`local_to_world_mm`. Drops the old geometry through
[`mirror_delete_pad_from_sketch`] and re-mints it through
[`mint_pad_entities`], so the new angle is honoured BY
CONSTRUCTION.

This is the whole point: per-shape layout knowledge (where a
chamfer anchor sits, where a round-rect arc centre sits) lives in
`mint` and nowhere else. Repositioning the four bbox corners with
[`mirror_move_pad_in_sketch`] is correct only for the shapes whose
entire outline IS those four corners — every parametric shape ends
up with rotated corners joined to un-rotated anchors, an outline
that is neither the old shape nor the new one. Teaching the corner
mover each shape's layout would make a third copy of those rules;
re-minting keeps one.

Re-minting also rewrites the `PadAttr` via `pad_attr_from_editor_pad`,
which is what keeps `attr.shape` — the field `oxide_bake::pad`
reads — in step with the editor's `pad.shape` after a flip swaps
the chamfer corners.

Returns `false` for a sketch-profile pad, where nothing was
regenerated: its copper is a traced loop, not a parametric shape,
so there is no layout to re-derive and the wildcard mint branch
would fabricate a bbox outline it never had. The caller owes the
user a warning in that case.

COST: constraints the user authored against the old outline
entities go with the old entities. The delete path already sweeps
them; a rotate is therefore a constraint-dropping edit. It is
undoable in one step — the history snapshot carries the whole
footprint file, sketch included.

## Source
Lines 196–211 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [is_sketch_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/is_sketch_profile_pad.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
| calls | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
| called_by | [fp_editor_set_selected_pad_rotation](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/fp_editor_set_selected_pad_rotation.md) |
| called_by | [with_selected_pad](/crates/oxide-app/src/app/handlers/dock/sch_library/footprint/pad/with_selected_pad.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
| called_by | [active_bar_flip_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_flip_selection.md) |
| called_by | [active_bar_rotate_selection](/crates/oxide-app/src/library/editor/footprint/updates/active_bar/active_bar_rotate_selection.md) |
