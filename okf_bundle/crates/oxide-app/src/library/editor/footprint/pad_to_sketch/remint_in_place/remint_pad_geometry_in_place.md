---
okf_version: "0.2"
type: Function
title: remint_pad_geometry_in_place
description: "Regenerate a pad's sidecar geometry through the same single owner"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place
language: rust
---

# remint_pad_geometry_in_place

Regenerate a pad's sidecar geometry through the same single owner

## Signature

```rust
pub fn remint_pad_geometry_in_place(pad: &mut EditorPad, footprint: &mut Footprint) -> bool
```

## Visibility

- `pub`

## Docstring

Regenerate a pad's sidecar geometry through the same single owner
[`remint_pad_geometry`] uses — [`mint_pad_entities`] — but writing
the freshly derived positions ONTO the existing entities instead of
replacing them.

This is the form a live drag needs. The pointer latches the id of
the Line or Point the user grabbed at press and streams one message
per cursor tick against it (`drag_tick_line` / `drag_tick_point` in
`canvas/input/pointer.rs`). A drop-and-re-mint invalidates that id
on the first tick, so the second tick addresses an entity that no
longer exists and the drag freezes one frame in while the cursor
keeps moving. Keeping the ids also keeps the constraints the user
authored against the outline, which a per-tick re-mint would wipe
on every mouse-move.

The layout rules still live only in `mint`: this mints a REFERENCE
copy of the same pad into a scratch footprint, under the same centre
id so the shape parameters it binds come out under the names the
real sketch already holds, and then copies geometry across the
old-id -> new-id correspondence.

Falls back to the full [`remint_pad_geometry`] when the entity
structure itself changed — a shape swap mints a different set, and
there is nothing to write onto. Returns `false` for a sketch-profile
pad exactly as [`remint_pad_geometry`] does, and the caller owes the
user the same warning.

## Source
Lines 44–95 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [remint_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place.md) |
| calls | [is_sketch_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/is_sketch_profile_pad.md) |
| calls | [mint_pad_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_pad_entities.md) |
| calls | [pair_sidecar_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pair_sidecar_entities.md) |
| calls | [pairing_covers_all_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/pairing_covers_all_geometry.md) |
| calls | [remint_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/remint_pad_geometry.md) |
| calls | [copy_entity_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/copy_entity_geometry.md) |
| calls | [record_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger.md) |
| called_by | [remint_dragged_pad](/crates/oxide-app/src/library/editor/footprint/updates/sketch/entities/remint_dragged_pad.md) |
