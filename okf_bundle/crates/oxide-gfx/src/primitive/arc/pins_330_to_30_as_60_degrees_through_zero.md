---
okf_version: "0.2"
type: Function
title: pins_330_to_30_as_60_degrees_through_zero
description: "The bug this whole normalization pass exists to fix, pinned as"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/pins_330_to_30_as_60_degrees_through_zero
language: rust
---

# pins_330_to_30_as_60_degrees_through_zero

The bug this whole normalization pass exists to fix, pinned as

## Signature

```rust
fn pins_330_to_30_as_60_degrees_through_zero()
```

## Decorators

- `test`

## Docstring

The bug this whole normalization pass exists to fix, pinned as
a unit test: a wrapped arc stored as `330° -> 30°` sweeps 60°
counter-clockwise through the 0°/360° seam — not the 300°
complement a naive signed `end - start` would suggest.
[test]

## Source
Lines 76–83 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
