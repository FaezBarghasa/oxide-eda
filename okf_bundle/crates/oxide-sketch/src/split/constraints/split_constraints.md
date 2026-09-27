---
okf_version: "0.2"
type: Function
title: split_constraints
description: "Rewrite every constraint in `sketch.constraints` against a"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/split_constraints
language: rust
---

# split_constraints

Rewrite every constraint in `sketch.constraints` against a

## Signature

```rust
pub(super) fn split_constraints(
    sketch: &SketchData,
    ctx: &SplitCtx,
) -> (Vec<Constraint>, Vec<ConstraintId>)
```

## Visibility

- `pub(super)`

## Docstring

Rewrite every constraint in `sketch.constraints` against a
completed split. Returns the full replacement list plus the ids of
any constraint dropped outright (today only a `Midpoint` naming the
retired line — see [`is_dropped_midpoint`]).

## Source
Lines 16–30 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| calls | [is_dropped_midpoint](/crates/oxide-sketch/src/split/constraints/is_dropped_midpoint.md) |
| calls | [split_constraint](/crates/oxide-sketch/src/split/constraints/split_constraint.md) |
| called_by | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
