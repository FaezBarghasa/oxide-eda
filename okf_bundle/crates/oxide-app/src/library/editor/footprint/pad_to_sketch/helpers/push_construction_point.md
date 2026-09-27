---
okf_version: "0.2"
type: Function
title: push_construction_point
description: "Push a `Point` entity flagged `construction = true` and return its"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_construction_point
language: rust
---

# push_construction_point

Push a `Point` entity flagged `construction = true` and return its

## Signature

```rust
pub(super) fn push_construction_point(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    x: f64,
    y: f64,
) -> SketchEntityId
```

## Visibility

- `pub(super)`

## Docstring

Push a `Point` entity flagged `construction = true` and return its
fresh ID.

## Source
Lines 33–44 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
| called_by | [mint_pad_corner_outline](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mint/mint_pad_corner_outline.md) |
