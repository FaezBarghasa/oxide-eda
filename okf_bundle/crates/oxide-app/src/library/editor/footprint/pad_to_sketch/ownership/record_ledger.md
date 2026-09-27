---
okf_version: "0.2"
type: Function
title: record_ledger
description: "Write the pad's current owned set onto its centre `PadAttr`, so the"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger
language: rust
---

# record_ledger

Write the pad's current owned set onto its centre `PadAttr`, so the

## Signature

```rust
pub(super) fn record_ledger(sketch: &mut SketchData, pad: &EditorPad, centre: SketchEntityId)
```

## Visibility

- `pub(super)`

## Docstring

Write the pad's current owned set onto its centre `PadAttr`, so the
answer survives the save + reopen that wipes every `EditorPad`
ownership field.

Called at the end of minting, which is the only moment the full set
is known from the volatile fields, and it is the ONLY write site —
nothing re-records afterwards. That is sound only because nothing
re-mints afterwards either: changing a pad's shape
(`fp_editor_set_selected_pad_shape`) writes `pad.shape` and never
calls back into `pad_to_sketch`, so the sketch still holds exactly
the geometry this ledger names. A delete drops the centre `PadAttr`
and the ledger with it.

If a re-mint path is ever added — a shape change that regenerates
geometry, say — it MUST route through `mint_shape_geometry_for` (or
call this) or the ledger will name entities that no longer exist
while the new ones are unowned.

## Source
Lines 130–140 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ownership](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.md) |
| calls | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
| called_by | [remint_pad_geometry_in_place](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/remint_in_place/remint_pad_geometry_in_place.md) |
