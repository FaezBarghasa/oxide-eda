---
okf_version: "0.2"
type: Function
title: mint_round_pad_geometry
description: "v0.24 Track A — mint a Round pad's geometry: 1 Circle entity"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_pad_geometry
language: rust
---

# mint_round_pad_geometry

v0.24 Track A — mint a Round pad's geometry: 1 Circle entity

## Signature

```rust
pub(super) fn mint_round_pad_geometry(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    pad: &mut EditorPad,
    centre_id: SketchEntityId,
)
```

## Visibility

- `pub(super)`

## Docstring

v0.24 Track A — mint a Round pad's geometry: 1 Circle entity
referencing the centre Point + a `diameter_<slug>` sketch
parameter recording the literal diameter for parametric edits.

## Source
Lines 70–87 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mint](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint.md) |
| calls | [bind_shape_param](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/bind_shape_param.md) |
| called_by | [mint_shape_geometry_for](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mint_shape_geometry_for.md) |
