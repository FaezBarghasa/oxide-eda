---
okf_version: "0.2"
type: Function
title: point_of
description: "Read a Point's raw x/y, panicking with the id when it isn't one."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of
language: rust
---

# point_of

Read a Point's raw x/y, panicking with the id when it isn't one.

## Signature

```rust
fn point_of(fp: &Footprint, id: SketchEntityId) -> (f64, f64)
```

## Docstring

Read a Point's raw x/y, panicking with the id when it isn't one.

## Source
Lines 291–304 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [mirror_move_oval_translates_anchor_sidecars](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_oval_translates_anchor_sidecars.md) |
| called_by | [mirror_move_profile_pad_owning_its_loop_applies_delta_once](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_owning_its_loop_applies_delta_once.md) |
| called_by | [mirror_move_roundrect_translates_anchors_and_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_roundrect_translates_anchors_and_arc_centres.md) |
| called_by | [roundrect_arc_centres](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/roundrect_arc_centres.md) |
