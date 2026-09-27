---
okf_version: "0.2"
type: Function
title: push_line
description: "Push a non-construction `Line` entity referencing `(start, end)`"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers/push_line
language: rust
---

# push_line

Push a non-construction `Line` entity referencing `(start, end)`

## Signature

```rust
pub(super) fn push_line(
    sketch: &mut SketchData,
    plane_id: PlaneId,
    start: SketchEntityId,
    end: SketchEntityId,
) -> SketchEntityId
```

## Visibility

- `pub(super)`

## Docstring

Push a non-construction `Line` entity referencing `(start, end)`
and return its fresh ID.

## Source
Lines 48–59 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/helpers.md) |
