---
okf_version: "0.2"
type: Function
title: commit_split
description: "Rewrite every other reference to `ctx.line` (constraints, arrays,"
resource: crates/oxide-sketch/src/split/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/mod/commit_split
language: rust
---

# commit_split

Rewrite every other reference to `ctx.line` (constraints, arrays,

## Signature

```rust
fn commit_split(
    sketch: &mut SketchData,
    ctx: &SplitCtx,
    line_idx: usize,
    mid_point: Entity,
    line_a: Entity,
    line_b: Entity,
) -> Vec<ConstraintId>
```

## Docstring

Rewrite every other reference to `ctx.line` (constraints, arrays,
pad profile seeds — see `split_line`'s doc table), then replace the
retired Line entity with the two new halves. Returns the ids of any
constraint dropped outright during the rewrite.

## Source
Lines 206–225 in `crates/oxide-sketch/src/split/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [split](/crates/oxide-sketch/src/split/mod.md) |
| calls | [split_constraints](/crates/oxide-sketch/src/split/constraints/split_constraints.md) |
| calls | [retarget_arrays](/crates/oxide-sketch/src/split/attr_refs/retarget_arrays.md) |
| calls | [retarget_pad_profiles](/crates/oxide-sketch/src/split/attr_refs/retarget_pad_profiles.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [split_line](/crates/oxide-sketch/src/split/mod/split_line.md) |
