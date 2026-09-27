---
okf_version: "0.2"
type: Function
title: mirror_move_profile_pad_owning_its_loop_applies_delta_once
description: "A profile pad that ALSO owns the loop's Points must take the move"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/mirror_move_profile_pad_owning_its_loop_applies_delta_once
language: rust
---

# mirror_move_profile_pad_owning_its_loop_applies_delta_once

A profile pad that ALSO owns the loop's Points must take the move

## Signature

```rust
fn mirror_move_profile_pad_owning_its_loop_applies_delta_once()
```

## Decorators

- `test`

## Docstring

A profile pad that ALSO owns the loop's Points must take the move
delta exactly once.

`mint_shape_geometry_for`'s catch-all arm sets `corner_entity_ids`
for `LibPadShape::Custom`, so a Custom pad minted through
`mirror_add_pad_to_sketch` whose `PadAttr` is a `SketchProfile` over
that same outline sits in BOTH the traced loop and the owned set.
The move translates the profile first and the owned set second, so
the shared Points used to take the delta twice and the outline
landed at double the offset — copper in the wrong place on the next
bake. Correctness was resting on an undocumented "these two sets are
disjoint" invariant that this configuration violates.
[test]

## Source
Lines 1159–1176 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests.md) |
| calls | [footprint_with_profile_pad](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/footprint_with_profile_pad.md) |
| calls | [mirror_move_pad_in_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_move_pad_in_sketch.md) |
| calls | [point_of](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/tests/point_of.md) |
