---
okf_version: "0.2"
type: Function
title: assert_corners_match_pad
description: "Every outline-corner `Point` must sit exactly where the pad's real"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad
language: rust
---

# assert_corners_match_pad

Every outline-corner `Point` must sit exactly where the pad's real

## Signature

```rust
fn assert_corners_match_pad(
    editor: &oxide_app::app::FootprintEditorState,
    pad: &EditorPad,
    when: &str,
)
```

## Docstring

Every outline-corner `Point` must sit exactly where the pad's real
copper corner is. Anything else is a construction outline that
disagrees with the copper it outlines.

The corner IDs are read off the PAD, live, not captured before the
transform. A frame transform re-mints the sidecar through
`pad_to_sketch::remint_pad_geometry`, so the outline entities that
exist afterwards are new ones — holding the pre-transform IDs would
assert entity identity, which is not the property that matters. The
property that matters is that the outline the pad currently points
at traces the copper.

## Source
Lines 462–496 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper](/crates/oxide-app/tests/footprint_pad_rotation/flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper.md) |
| called_by | [properties_panel_rotation_moves_the_sketch_outline_corners](/crates/oxide-app/tests/footprint_pad_rotation/properties_panel_rotation_moves_the_sketch_outline_corners.md) |
| called_by | [rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper](/crates/oxide-app/tests/footprint_pad_rotation/rotate_moves_the_sketch_outline_corners_to_match_the_turned_copper.md) |
