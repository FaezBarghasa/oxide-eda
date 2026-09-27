---
okf_version: "0.2"
type: Function
title: point_param
description: "Raw (unclamped) parametric position of `point`'s coordinates"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/point_param
language: rust
---

# point_param

Raw (unclamped) parametric position of `point`'s coordinates

## Signature

```rust
fn point_param(sketch: &SketchData, point: SketchEntityId, ctx: &SplitCtx) -> f64
```

## Docstring

Raw (unclamped) parametric position of `point`'s coordinates
projected onto the original line. An unresolvable `point` id (a
pre-existing malformed sketch — the id doesn't resolve to a Point)
falls back to `0.0`, landing the constraint on `line_a`; that's a
safe default, not a correctness claim about a sketch that was
already broken before this split ran.

## Source
Lines 152–157 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [entity_point_xy](/crates/oxide-sketch/src/split/mod/entity_point_xy.md) |
| called_by | [split_point_on_line](/crates/oxide-sketch/src/split/constraints/split_point_on_line.md) |
