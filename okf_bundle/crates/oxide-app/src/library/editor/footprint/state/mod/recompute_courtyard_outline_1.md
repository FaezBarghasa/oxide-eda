---
okf_version: "0.2"
type: Function
title: recompute_courtyard_outline
description: v0.27 — outline-following courtyard. Builds a polygon for
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/recompute_courtyard_outline_1
language: rust
---

# recompute_courtyard_outline

v0.27 — outline-following courtyard. Builds a polygon for

## Signature

```rust
pub fn recompute_courtyard_outline(&mut self) -> bool
```

## Visibility

- `pub`

## Docstring

v0.27 — outline-following courtyard. Builds a polygon for
each pad's bbox, unions the lot via the sketch geom module,
offsets the union by `COURTYARD_SLACK_MM`, and stores the
result on `courtyard_outline_mm`. Replaces the bbox-based
`recompute_courtyard` for users who want the courtyard to
hug the actual pad cluster rather than its enclosing
rectangle. Returns `true` when a polygon was produced;
`false` when there are no pads or the boolean union failed.

## Source
Lines 553–628 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
| calls | [polygon_op](/crates/oxide-sketch/src/geom/boolean_general/polygon_op.md) |
| calls | [offset_polygon](/crates/oxide-sketch/src/geom/offset/offset_polygon.md) |
