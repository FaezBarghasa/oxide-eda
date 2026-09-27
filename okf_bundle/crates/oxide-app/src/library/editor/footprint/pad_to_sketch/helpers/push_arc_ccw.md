---
okf_version: "0.2"
type: Function
title: push_arc_ccw
description: "Push a non-construction `Arc` entity (CCW sweep) and return its"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_arc_ccw
language: rust
---

# push_arc_ccw

Push a non-construction `Arc` entity (CCW sweep) and return its

## Signature

```rust
pub(super) fn push_arc_ccw(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    center: SketchEntityId,
    start: SketchEntityId,
    end: SketchEntityId,
) -> SketchEntityId
```

## Visibility

- `pub(super)`

## Docstring

Push a non-construction `Arc` entity (CCW sweep) and return its
fresh ID.

## Source
Lines 79–98 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
| called_by | [mint_oval_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_oval_pad_geometry.md) |
| called_by | [mint_round_rect_pad_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_round_rect_pad_geometry.md) |
