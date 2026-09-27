---
okf_version: "0.2"
type: Function
title: flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper
description: "Same defect on the Flip arm — it negates the angle, so"
resource: crates/oxide-app/tests/footprint_pad_rotation.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_rotation/flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper
language: rust
---

# flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper

Same defect on the Flip arm — it negates the angle, so

## Signature

```rust
fn flip_moves_the_sketch_outline_corners_to_match_the_mirrored_copper()
```

## Decorators

- `test`

## Docstring

Same defect on the Flip arm — it negates the angle, so
`rotated_corners_mm()` moves and the outline has to follow.
[test]

## Source
Lines 372–388 in `crates/oxide-app/tests/footprint_pad_rotation.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_rotation](/crates/oxide-app/tests/footprint_pad_rotation.md) |
| calls | [sketched_pad_fixture](/crates/oxide-app/tests/footprint_pad_rotation/sketched_pad_fixture.md) |
| calls | [dispatch](/crates/oxide-app/tests/footprint_pad_rotation/dispatch.md) |
| calls | [assert_corners_match_pad](/crates/oxide-app/tests/footprint_pad_rotation/assert_corners_match_pad.md) |
