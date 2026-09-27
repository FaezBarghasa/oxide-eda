---
okf_version: "0.2"
type: Function
title: is_inside
description: "`true` when `p` is on the inside (left side) of the directed"
resource: crates/oxide-sketch/src/geom/boolean.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/boolean/is_inside
language: rust
---

# is_inside

`true` when `p` is on the inside (left side) of the directed

## Signature

```rust
fn is_inside(p: Point2, a: Point2, b: Point2) -> bool
```

## Docstring

`true` when `p` is on the inside (left side) of the directed
edge `a → b`. Inclusive — points exactly on the edge count as
inside so the polygon's own vertices on a clip boundary aren't
dropped.

## Source
Lines 102–105 in `crates/oxide-sketch/src/geom/boolean.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [boolean](/crates/oxide-sketch/src/geom/boolean.md) |
| called_by | [clip_against_edge](/crates/oxide-sketch/src/geom/boolean/clip_against_edge.md) |
