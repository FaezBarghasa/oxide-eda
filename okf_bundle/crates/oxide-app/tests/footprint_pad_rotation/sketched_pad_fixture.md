---
okf_version: "0.2"
type: Function
title: sketched_pad_fixture
description: "A footprint editor holding one selected `Rect` pad at the origin"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture
language: rust
---

# sketched_pad_fixture

A footprint editor holding one selected `Rect` pad at the origin

## Signature

```rust
fn sketched_pad_fixture(stem: &str, size_mm: (f64, f64) -> (Oxide, PathBuf)
```

## Docstring

A footprint editor holding one selected `Rect` pad at the origin
with its sketch outline already minted.

## Source
Lines 414–449 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
| called_by | [flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper.md) |
| called_by | [properties_panel_rotation_moves_the_sketch_outline_corners](/crates/oxide-app/tests/footprint_pad_rotation/properties_panel_rotation_moves_the_sketch_outline_corners.md) |
| called_by | [rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper](/crates/oxide-app/tests/footprint_pad_rotation/rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper.md) |
