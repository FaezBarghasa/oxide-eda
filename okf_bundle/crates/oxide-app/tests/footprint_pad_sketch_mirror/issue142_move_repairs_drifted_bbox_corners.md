---
okf_version: "0.2"
type: Function
title: issue142_move_repairs_drifted_bbox_corners
description: "A move re-states the bbox corners absolutely, so drift self-heals."
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_move_repairs_drifted_bbox_corners
language: rust
---

# issue142_move_repairs_drifted_bbox_corners

A move re-states the bbox corners absolutely, so drift self-heals.

## Signature

```rust
fn issue142_move_repairs_drifted_bbox_corners()
```

## Decorators

- `test`

## Docstring

A move re-states the bbox corners absolutely, so drift self-heals.

Routing move through a single centre-derived delta made it purely
cumulative: any mismatch between the pad's declared size and its
sketch outline became permanent, and repeated moves accumulated
rounding in every anchor. The four bbox corners are the only owned
Points whose position is fully derivable from `Pad`, so they are
re-asserted from `bbox_mm()` after the delta pass.
[test]

## Source
Lines 228–254 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
