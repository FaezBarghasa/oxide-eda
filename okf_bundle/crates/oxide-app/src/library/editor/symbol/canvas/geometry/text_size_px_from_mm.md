---
okf_version: "0.2"
type: Function
title: text_size_px_from_mm
description: Rendered em size for hit-testing — the same rule the draw path uses.
resource: crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/canvas/geometry/text_size_px_from_mm
language: rust
---

# text_size_px_from_mm

Rendered em size for hit-testing — the same rule the draw path uses.

## Signature

```rust
pub(super) fn text_size_px_from_mm(size_mm: f32, scale: f32) -> f32
```

## Visibility

- `pub(super)`

## Docstring

Rendered em size for hit-testing — the same rule the draw path uses.

This used to be a second copy of the sizing arithmetic with the clamp
bounds written out again. The two agreed, so nothing was visibly broken;
what they were was two places to edit. Change the draw path's bounds and
the hit box silently keeps the old ones, and a click near a pin label
starts landing where the text no longer is.

## Source
Lines 17–19 in `crates/oxide-app/src/library/editor/symbol/canvas/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/symbol/canvas/geometry.md) |
| called_by | [draw_origin_marker](/crates/oxide-app/src/library/editor/symbol/canvas/draw/background/draw_origin_marker.md) |
