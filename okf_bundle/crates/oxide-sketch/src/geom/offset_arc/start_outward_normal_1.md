---
okf_version: "0.2"
type: Function
title: start_outward_normal
description: Outward unit-perpendicular direction at the START of the
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc/start_outward_normal_1
language: rust
---

# start_outward_normal

Outward unit-perpendicular direction at the START of the

## Signature

```rust
fn start_outward_normal(&self, polygon_ccw: bool) -> (f64, f64)
```

## Docstring

Outward unit-perpendicular direction at the START of the
element. For a Line, this is the perpendicular to the
segment direction. For an Arc, it's the radial direction
(outward from centre = positive offset direction for CCW).

## Source
Lines 64–83 in `crates/oxide-sketch/src/geom/offset_arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset_arc](/crates/oxide-sketch/src/geom/offset_arc.md) |
| calls | [unit_perp_outward](/crates/oxide-sketch/src/geom/offset_arc/unit_perp_outward.md) |
