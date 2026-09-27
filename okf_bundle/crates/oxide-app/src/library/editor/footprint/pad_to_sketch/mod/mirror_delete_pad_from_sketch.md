---
okf_version: "0.2"
type: Function
title: mirror_delete_pad_from_sketch
description: "v0.15 — when a pad is deleted in Pads mode, also drop its backing"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch
language: rust
---

# mirror_delete_pad_from_sketch

v0.15 — when a pad is deleted in Pads mode, also drop its backing

## Signature

```rust
pub fn mirror_delete_pad_from_sketch(pad: &EditorPad, footprint: &mut Footprint)
```

## Visibility

- `pub`

## Docstring

v0.15 — when a pad is deleted in Pads mode, also drop its backing
sketch entity (and any constraints that referenced it).

v0.24 Track A — also drop linked Circle / Arc entities and any
sketch parameters keyed by the centre-Point UUID slug. RoundRect's
anchor / inset-corner Points are pulled into the drop set via a
secondary sweep — they're referenced indirectly by Arcs whose
`center` is the inset corner.

## Source
Lines 521–605 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| calls | [id_slug](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/id_slug.md) |
| called_by | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| called_by | [mirror_delete_drops_constraints_on_owned_corners](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_constraints_on_owned_corners.md) |
| called_by | [mirror_delete_drops_per_corner_unlink_parameter](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_drops_per_corner_unlink_parameter.md) |
| called_by | [mirror_delete_pad_drops_constraints_on_the_whole_entity_set](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_constraints_on_the_whole_entity_set.md) |
| called_by | [mirror_delete_pad_drops_sketch_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_delete_pad_drops_sketch_entity.md) |
| called_by | [apply_footprint_clipboard_op](/crates/oxide-app/src/library/editor/footprint/updates/mod/apply_footprint_clipboard_op.md) |
| called_by | [delete_selected](/crates/oxide-app/src/library/editor/footprint/updates/selection/delete_selected.md) |
| called_by | [issue142_delete_does_not_eat_user_geometry_sharing_an_anchor](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_delete_does_not_eat_user_geometry_sharing_an_anchor.md) |
| called_by | [issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_deleting_one_duplicate_numbered_pad_keeps_the_others_copper.md) |
| called_by | [issue142_owned_ledger_survives_a_real_serde_round_trip](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_owned_ledger_survives_a_real_serde_round_trip.md) |
| called_by | [issue142_reopened_pad_delete_removes_its_geometry](/crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_reopened_pad_delete_removes_its_geometry.md) |
