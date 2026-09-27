---
okf_version: "0.2"
type: Function
title: rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle
description: "THE INVARIANT (a). One `ActiveBarRotateSelection` on the #390 repro"
resource: crates/oxide-app/tests/footprint_pad_remint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_remint/rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle
language: rust
---

# rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle

THE INVARIANT (a). One `ActiveBarRotateSelection` on the #390 repro

## Signature

```rust
fn rotate_leaves_the_sketch_equal_to_a_fresh_mint_at_the_new_angle()
```

## Decorators

- `test`

## Docstring

THE INVARIANT (a). One `ActiveBarRotateSelection` on the #390 repro
must leave the sketch in the state a from-scratch mint of the same
pad at the same angle produces.

The named coordinate: the NE chamfer anchor is minted in the pad
frame at (xmax − r, ymin) = (0.75, −0.5). Taken through a 90° frame
about the origin that is (0.5, 0.75). Re-placing only the four bbox
corners leaves it at (0.75, −0.5) — the corners turn, the chamfer
does not, and the outline is a mix of two frames that is neither
the old shape nor the new one.
[test]

## Source
Lines 166–211 in `crates/oxide-app/tests/footprint_pad_remint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_remint](/crates/oxide-app/tests/footprint_pad_remint.md) |
| calls | [editor_with_minted_pad](/crates/oxide-app/tests/footprint_pad_remint/editor_with_minted_pad.md) |
| calls | [chamfered_repro_pad](/crates/oxide-app/tests/footprint_pad_remint/chamfered_repro_pad.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_remint/dispatch.md) |
| calls | [sidecar_point](/crates/oxide-app/tests/footprint_pad_remint/sidecar_point.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
