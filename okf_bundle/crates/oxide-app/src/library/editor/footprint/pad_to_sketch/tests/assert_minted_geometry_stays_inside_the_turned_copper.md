---
okf_version: "0.2"
type: Function
title: assert_minted_geometry_stays_inside_the_turned_copper
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/assert_minted_geometry_stays_inside_the_turned_copper
language: rust
---

# assert_minted_geometry_stays_inside_the_turned_copper

## Signature

```rust
fn assert_minted_geometry_stays_inside_the_turned_copper(shape: LibPadShape)
```

## Source
Lines 673–697 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| calls | [minted_points_in_pad_frame](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/minted_points_in_pad_frame.md) |
| called_by | [rotated_chamfered_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_chamfered_mints_geometry_that_closes.md) |
| called_by | [rotated_oval_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_oval_mints_geometry_that_closes.md) |
| called_by | [rotated_round_rect_mints_geometry_that_closes](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/rotated_round_rect_mints_geometry_that_closes.md) |
