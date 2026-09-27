---
okf_version: "0.2"
type: Function
title: split_point_on_line
description: "`PointOnLine` — re-points to whichever half the constrained point"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/split_point_on_line
language: rust
---

# split_point_on_line

`PointOnLine` — re-points to whichever half the constrained point

## Signature

```rust
fn split_point_on_line(
    c: &Constraint,
    sketch: &SketchData,
    ctx: &SplitCtx,
) -> Option<Vec<Constraint>>
```

## Docstring

`PointOnLine` — re-points to whichever half the constrained point
actually falls on, compared against the split parameter `t`.

`PointOnLine`'s residual is the signed perpendicular distance to
the INFINITE line through `line`'s endpoints
(`solver::residuals::point_on::point_on_line`), not a
segment-bounded containment check, so leaving it pointed at either
half wouldn't literally fail to resolve. What makes re-pointing the
right choice is that the two halves stop being collinear once
either can hinge independently around `mid` on a later solve — "the
infinite line through `line_a`" and "the infinite line through
`line_b`" are no longer the same line. Following the point to
whichever half it currently sits nearest keeps the constraint
pinning the SAME relationship the user authored, instead of quietly
re-defining it against a line that may end up passing somewhere
else entirely.

## Source
Lines 121–144 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [point_param](/crates/oxide-sketch/src/split/constraints/point_param.md) |
| called_by | [split_constraint](/crates/oxide-sketch/src/split/constraints/split_constraint.md) |
