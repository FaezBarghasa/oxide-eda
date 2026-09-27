---
okf_version: "0.2"
type: Function
title: retarget_relational
description: "`Parallel` / `Perpendicular` / `Angle` / `EqualLength` /"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/retarget_relational
language: rust
---

# retarget_relational

`Parallel` / `Perpendicular` / `Angle` / `EqualLength` /

## Signature

```rust
fn retarget_relational(c: &Constraint, ctx: &SplitCtx) -> Constraint
```

## Docstring

`Parallel` / `Perpendicular` / `Angle` / `EqualLength` /
`TangentLineArc` / `SymmetricAboutLine` / `DistancePtLine` —
re-pointed onto `line_a` only; everything else (including a
`Midpoint` NOT naming the retired line) passes through unchanged.

Each of these relates the split line to a SECOND, independent
entity (another line, an arc, or a scalar target). Duplicating
would assert the identical numeric relationship for two different
physical segments against one external reference — generally
infeasible the instant the mid point or the partner entity moves
independently after the split (two segments can't both equal
`l2`'s length, or both sit at `target` degrees from `l2`, unless
the split happened to land exactly at the midpoint by coincidence).
Re-pointing to one half preserves exactly what the original
constraint asserted about the one physical line it still
describes, and leaves the other half's extra DOF free — which is
the expected price of turning one rigid line into a two-segment
hinge at the mid point.

## Source
Lines 177–219 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [sub](/crates/oxide-sketch/src/solver/math/sub.md) |
