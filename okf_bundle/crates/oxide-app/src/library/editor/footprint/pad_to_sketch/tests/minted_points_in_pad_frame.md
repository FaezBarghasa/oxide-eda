---
okf_version: "0.2"
type: Function
title: minted_points_in_pad_frame
description: "Every `Point` in `fp`'s sketch, expressed in the pad's own frame."
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_points_in_pad_frame
language: rust
---

# minted_points_in_pad_frame

Every `Point` in `fp`'s sketch, expressed in the pad's own frame.

## Signature

```rust
fn minted_points_in_pad_frame(fp: &Footprint, pad: &EditorPad) -> Vec<(f64, f64)>
```

## Docstring

Every `Point` in `fp`'s sketch, expressed in the pad's own frame.

## Source
Lines 660–671 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| called_by | [assert_minted_geometry_stays_inside_the_turned_copper](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_minted_geometry_stays_inside_the_turned_copper.md) |
