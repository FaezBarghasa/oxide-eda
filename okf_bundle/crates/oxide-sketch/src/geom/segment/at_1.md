---
okf_version: "0.2"
type: Function
title: at
description: "Linearly interpolate along the segment. `t = 0` returns `a`,"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/at_1
language: rust
---

# at

Linearly interpolate along the segment. `t = 0` returns `a`,

## Signature

```rust
pub fn at(&self, t: f64) -> Point2
```

## Visibility

- `pub`

## Docstring

Linearly interpolate along the segment. `t = 0` returns `a`,
`t = 1` returns `b`. Outside `[0, 1]` extrapolates onto the
underlying line.

## Source
Lines 27–32 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
