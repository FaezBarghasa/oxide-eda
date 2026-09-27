---
okf_version: "0.2"
type: Function
title: left_turn_or_colinear
description: "`true` when the three points `a`, `b`, `c` form a left turn (or"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/left_turn_or_colinear
language: rust
---

# left_turn_or_colinear

`true` when the three points `a`, `b`, `c` form a left turn (or

## Signature

```rust
pub fn left_turn_or_colinear(a: Point2, b: Point2, c: Point2) -> bool
```

## Visibility

- `pub`

## Docstring

`true` when the three points `a`, `b`, `c` form a left turn (or
`b` is colinear and on the segment from `a` to `c`). Useful for
convex-hull-style "remove right turns" loops where colinear-on-
segment points should NOT be removed.

## Source
Lines 287–289 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
