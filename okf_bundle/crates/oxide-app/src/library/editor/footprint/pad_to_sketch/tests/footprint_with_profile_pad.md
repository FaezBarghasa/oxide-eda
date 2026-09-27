---
okf_version: "0.2"
type: Function
title: footprint_with_profile_pad
description: Build a footprint whose sketch holds a closed rectangle profile and a
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/footprint_with_profile_pad
language: rust
---

# footprint_with_profile_pad

Build a footprint whose sketch holds a closed rectangle profile and a

## Signature

```rust
fn footprint_with_profile_pad() -> (Footprint, EditorPad, [SketchEntityId; 4])
```

## Docstring

Build a footprint whose sketch holds a closed rectangle profile and a
centre Point carrying a `SketchProfile` PadAttr seeded from one of the
rectangle's Lines — the shape produced by "Make Pad from Profile".

Returns the pad (linked to the centre) and the four profile-corner ids
in sw, se, ne, nw order.

## Source
Lines 178–237 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [editor_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/editor_pad.md) |
| called_by | [mirror_move_profile_pad_owning_its_loop_applies_delta_once](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_owning_its_loop_applies_delta_once.md) |
| called_by | [mirror_move_profile_pad_translates_profile_geometry](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_translates_profile_geometry.md) |
