---
okf_version: "0.2"
type: Function
title: rotated_wraparound_arc_hit_test_and_draw_sweep_agree
description: Rotating a 0°-crossing arc must keep hit-test and the CPU draw
resource: crates/oxide-app/src/library/editor/symbol/state/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/state/tests/rotated_wraparound_arc_hit_test_and_draw_sweep_agree
language: rust
---

# rotated_wraparound_arc_hit_test_and_draw_sweep_agree

Rotating a 0°-crossing arc must keep hit-test and the CPU draw

## Signature

```rust
fn rotated_wraparound_arc_hit_test_and_draw_sweep_agree()
```

## Decorators

- `test`

## Docstring

Rotating a 0°-crossing arc must keep hit-test and the CPU draw
arm's tessellated sweep in agreement — the exact bug class this
normalization pass fixes. Starts as `270 -> 90` (a 180° arc
spanning the world's left half, CCW from 270° through 0° to 90°);
rotating it CW 90° stores `180 -> 0` (`rotation.rs`'s independent
`rem_euclid`-based normalization produces a wrapped, `end < start`
pair here — the exact form the CPU draw path used to mis-render).
[test]

## Source
Lines 841–895 in `crates/oxide-app/src/library/editor/symbol/state/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/symbol/state/tests.md) |
| calls | [arc_symbol](/crates/oxide-app/src/library/editor/symbol/state/tests/arc_symbol.md) |
| calls | [rotate_selected](/crates/oxide-app/src/library/editor/symbol/state/rotation/rotate_selected.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
