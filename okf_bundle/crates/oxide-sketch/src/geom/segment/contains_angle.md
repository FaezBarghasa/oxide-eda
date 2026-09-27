---
okf_version: "0.2"
type: Function
title: contains_angle
description: "`true` when the angle `theta` (any reference frame, will be"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/contains_angle
language: rust
---

# contains_angle

`true` when the angle `theta` (any reference frame, will be

## Signature

```rust
impl Arc2 { pub fn contains_angle(&self, theta: f64) -> bool }
```

## Visibility

- `pub`

## Docstring

`true` when the angle `theta` (any reference frame, will be
normalised) lies within the arc's sweep — inclusive on both
ends. Handles arcs that cross the seam.

## Source
Lines 90–122 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
