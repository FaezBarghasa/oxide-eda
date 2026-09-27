---
okf_version: "0.2"
type: Class
title: Arc2
description: "2D circular arc. `start_rad` and `end_rad` are angles in radians"
resource: crates/oxide-sketch/src/geom/segment.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/segment/Arc2
language: rust
---

# Arc2

2D circular arc. `start_rad` and `end_rad` are angles in radians

## Signature

```rust
pub struct Arc2
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq)`

## Visibility

- `pub`

## Docstring

2D circular arc. `start_rad` and `end_rad` are angles in radians
measured from the centre using the standard atan2 convention
(positive X axis = 0, CCW positive). `sweep_ccw = true` means the
arc walks CCW from `start_rad` to `end_rad`. The arc may cross
the seam at ±π — the angular containment check handles wrap-around.
[derive(Debug, Clone, Copy, PartialEq)]

## Methods

- `center`
- `radius`
- `start_rad`
- `end_rad`
- `sweep_ccw`

## Source
Lines 68–74 in `crates/oxide-sketch/src/geom/segment.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [segment](/crates/oxide-sketch/src/geom/segment.md) |
