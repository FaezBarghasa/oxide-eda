---
okf_version: "0.2"
type: Function
title: mirror_move_roundrect_translates_anchors_and_arc_centres
description: Moving a RoundRect pad must carry its arc anchors and inset
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres
language: rust
---

# mirror_move_roundrect_translates_anchors_and_arc_centres

Moving a RoundRect pad must carry its arc anchors and inset

## Signature

```rust
fn mirror_move_roundrect_translates_anchors_and_arc_centres()
```

## Decorators

- `test`

## Docstring

Moving a RoundRect pad must carry its arc anchors and inset
arc-centres along.

Regression: `mirror_move_pad_in_sketch` repositioned only the centre
Point and the four `corner_entity_ids`. RoundRect additionally mints
8 edge anchors + 4 inset arc-centres, all NON-construction, so they
stayed at the old coordinates and the bake emitted copper from the
stranded geometry. Nothing downstream repaired it —
`sync_pads_to_primitive` copies attributes only, it never re-mints.
[test]

## Source
Lines 325–373 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [sidecar](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/sidecar.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [dedup](/crates/oxide-sketch/src/geom/simplify/dedup.md) |
| calls | [point_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
