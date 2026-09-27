---
okf_version: "0.2"
type: Function
title: v027_sketch_line_drag_resizes_rect_pad_bbox
description: v0.27 — dragging a pad-outline edge in Sketch mode must propagate
resource: crates/oxide-app/tests/regression/library_placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_placement/v027_sketch_line_drag_resizes_rect_pad_bbox
language: rust
---

# v027_sketch_line_drag_resizes_rect_pad_bbox

v0.27 — dragging a pad-outline edge in Sketch mode must propagate

## Signature

```rust
fn v027_sketch_line_drag_resizes_rect_pad_bbox()
```

## Decorators

- `test`

## Docstring

v0.27 — dragging a pad-outline edge in Sketch mode must propagate
through to `pad.size_mm` / `pad.position_mm`. Pre-fix: the sketch
outline visibly resized but the literal pad bbox never updated,
so the rendered pad copper underneath the moving line stayed put.
[test]

## Source
Lines 670–765 in `crates/oxide-app/tests/regression/library_placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_placement](/crates/oxide-app/tests/regression/library_placement.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
