---
okf_version: "0.2"
type: Function
title: retarget_arrays
description: "An array's `source` (and Polar's `center`) must resolve to a Point"
resource: crates/oxide-sketch/src/split/attr_refs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/attr_refs/retarget_arrays
language: rust
---

# retarget_arrays

An array's `source` (and Polar's `center`) must resolve to a Point

## Signature

```rust
pub(super) fn retarget_arrays(sketch: &mut SketchData, ctx: &SplitCtx)
```

## Visibility

- `pub(super)`

## Docstring

An array's `source` (and Polar's `center`) must resolve to a Point
carrying a `PadAttr` to bake at all — `bake_one_pad` looks it up
via `point_xy`, which a Line id never satisfies. So this only fires
on sketch data that was already malformed before the split; it is
still rewritten rather than left dangling, the same judgment call
`constraints::point_param` makes for an unresolvable point.

## Source
Lines 20–32 in `crates/oxide-sketch/src/split/attr_refs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr_refs](/crates/oxide-sketch/src/split/attr_refs.md) |
| calls | [retarget](/crates/oxide-sketch/src/split/attr_refs/retarget.md) |
| called_by | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
