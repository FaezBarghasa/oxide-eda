---
okf_version: "0.2"
type: Function
title: mirror_shape
description: "`oxide_bake::pad` reads `PadAttr::shape`, so the sketch copy of"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_shape
language: rust
---

# mirror_shape

`oxide_bake::pad` reads `PadAttr::shape`, so the sketch copy of

## Signature

```rust
fn mirror_shape(current: &mut SkPadShape, shape: &LibPadShape)
```

## Docstring

`oxide_bake::pad` reads `PadAttr::shape`, so the sketch copy of
the shape IS what gets baked. Leaving it out of the mirror let a
flip swap the editor pad's chamfer corners while the sketch — and
therefore the bake — kept the pre-flip ones: two representations,
two answers, and the part does not seat.

A `SketchProfile` custom shape is never overwritten. That variant
is not derived from `pad.shape` at all — it names the traced loop
that IS the pad's copper, and `map_shape` would replace it with a
static point list, silently severing the profile link.

## Source
Lines 65–73 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr.md) |
| calls | [map_shape](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/map_shape.md) |
| called_by | [mirror_pad_attrs_into_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/attr/mirror_pad_attrs_into_sketch.md) |
