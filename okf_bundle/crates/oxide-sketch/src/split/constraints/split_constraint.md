---
okf_version: "0.2"
type: Function
title: split_constraint
description: "Rewrite one constraint (already confirmed not a dropped `Midpoint`)"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/split_constraint
language: rust
---

# split_constraint

Rewrite one constraint (already confirmed not a dropped `Midpoint`)

## Signature

```rust
fn split_constraint(c: &Constraint, sketch: &SketchData, ctx: &SplitCtx) -> Vec<Constraint>
```

## Docstring

Rewrite one constraint (already confirmed not a dropped `Midpoint`)
against a completed split. Returns 1 or 2 replacement constraints.
A constraint whose fields never named `line` (including every kind
that can only reference a Point / Arc / Circle — `Coincident`,
`DistancePtPt`, `Fixed`, `PointOnArc`, `DistancePtCircle`,
`EqualRadius`, `TangentArcArc`, `SymmetricAboutPoint`) is returned
unchanged: none of their fields can dangle from this split, since
the original Line's endpoint Points keep their ids and coordinates
untouched — only the Line entity itself is retired.

## Source
Lines 51–59 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [split_duplicated](/crates/oxide-sketch/src/split/constraints/split_duplicated.md) |
| calls | [split_point_on_line](/crates/oxide-sketch/src/split/constraints/split_point_on_line.md) |
| called_by | [split_constraints](/crates/oxide-sketch/src/split/constraints/split_constraints.md) |
