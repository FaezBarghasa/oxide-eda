---
okf_version: "0.2"
type: Function
title: point_xy_of
description: "Raw x/y of a sketch `Point` entity, straight off `SketchData` —"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/point_xy_of
language: rust
---

# point_xy_of

Raw x/y of a sketch `Point` entity, straight off `SketchData` —

## Signature

```rust
fn point_xy_of(sketch: &SketchData, id: SketchEntityId) -> Option<(f64, f64)>
```

## Docstring

Raw x/y of a sketch `Point` entity, straight off `SketchData` —
the pre-solve authored position, which is what the mirror mutates.

## Source
Lines 438–447 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| called_by | [translate_profile_with_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/translate_profile_with_pad.md) |
