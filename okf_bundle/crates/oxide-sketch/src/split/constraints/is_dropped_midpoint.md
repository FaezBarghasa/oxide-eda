---
okf_version: "0.2"
type: Function
title: is_dropped_midpoint
description: "`Midpoint` names the retired line — its referent is destroyed by"
resource: crates/oxide-sketch/src/split/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/split/constraints/is_dropped_midpoint
language: rust
---

# is_dropped_midpoint

`Midpoint` names the retired line — its referent is destroyed by

## Signature

```rust
fn is_dropped_midpoint(c: &Constraint, ctx: &SplitCtx) -> bool
```

## Docstring

`Midpoint` names the retired line — its referent is destroyed by
the split (the original line's midpoint is the midpoint of NEITHER
half), unlike `Parallel` / `EqualLength` / etc. where `line_a`
genuinely still holds the relation. Dropped rather than re-pointed;
the id is surfaced via `SplitResult::dropped_constraints` instead
of silently vanishing.

## Source
Lines 38–40 in `crates/oxide-sketch/src/split/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-sketch/src/split/constraints.md) |
| called_by | [split_constraints](/crates/oxide-sketch/src/split/constraints/split_constraints.md) |
