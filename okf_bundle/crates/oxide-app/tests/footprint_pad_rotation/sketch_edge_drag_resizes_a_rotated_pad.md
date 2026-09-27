---
okf_version: "0.2"
type: Function
title: sketch_edge_drag_resizes_a_rotated_pad
description: The v0.27 sketch-line-edge-drag → pad-resize propagation
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/sketch_edge_drag_resizes_a_rotated_pad
language: rust
---

# sketch_edge_drag_resizes_a_rotated_pad

The v0.27 sketch-line-edge-drag → pad-resize propagation

## Signature

```rust
fn sketch_edge_drag_resizes_a_rotated_pad()
```

## Decorators

- `test`

## Docstring

The v0.27 sketch-line-edge-drag → pad-resize propagation
classified the dragged line against the un-rotated `bbox_mm()`.
Once the outline is minted rotated, a turned pad's edges are
diagonal (or, at 90°, axis-aligned but with W/H swapped relative
to the un-rotated box) and the classification rejects every one of
them — the propagation silently no-ops and the user sees the line
move while the copper underneath does nothing.
[test]

## Source
Lines 506–594 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
