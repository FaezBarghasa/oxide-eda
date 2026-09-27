---
okf_version: "0.2"
type: Function
title: owned_sketch_entities
description: "Every sketch entity `pad` owns, deduplicated, centre first."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities
language: rust
---

# owned_sketch_entities

Every sketch entity `pad` owns, deduplicated, centre first.

## Signature

```rust
pub(super) fn owned_sketch_entities(pad: &EditorPad, sketch: &SketchData) -> Vec<SketchEntityId>
```

## Visibility

- `pub(super)`

## Docstring

Every sketch entity `pad` owns, deduplicated, centre first.

Seeded from the three ownership fields, then expanded FORWARD once:
a seeded `Line` also owns its `start` / `end`, an `Arc` its
`center` / `start` / `end`, a `Circle` its `center`. One pass is
enough because Points are leaves, and forward expansion alone is
complete for every shape — RoundRect's four sidecar Arcs yield
exactly its 4 inset centres + 8 edge anchors, while Oval and
Chamfered record their anchors directly.

Deliberately NOT expanded in reverse: a user-drawn line that
happens to snap to a pad corner is not the pad's to drag around.
(Delete does sweep in reverse, but that is its own decision —
deleting a pad should take the dangling geometry with it.)

## Source
Lines 49–99 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ownership](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.md) |
| calls | [persisted_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/persisted_ledger.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
| called_by | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [record_ledger](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/record_ledger.md) |
| called_by | [assert_in_place_remint_matches_fresh_mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_in_place_remint_matches_fresh_mint.md) |
| called_by | [owned_point_positions](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_point_positions.md) |
| called_by | [owned_set_excludes_ids_with_no_live_entity](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/owned_set_excludes_ids_with_no_live_entity.md) |
