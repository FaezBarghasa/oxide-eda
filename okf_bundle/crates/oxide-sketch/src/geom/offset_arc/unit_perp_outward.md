---
okf_version: "0.2"
type: Function
title: unit_perp_outward
description: Unit perpendicular pointing OUT of a CCW polygon (right of edge
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc/unit_perp_outward
language: rust
---

# unit_perp_outward

Unit perpendicular pointing OUT of a CCW polygon (right of edge

## Signature

```rust
fn unit_perp_outward(a: Point2, b: Point2, ccw: bool) -> (f64, f64)
```

## Docstring

Unit perpendicular pointing OUT of a CCW polygon (right of edge
direction). Flipped for CW.

## Source
Lines 104–110 in `crates/oxide-sketch/src/geom/offset_arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset_arc](/crates/oxide-sketch/src/geom/offset_arc.md) |
| called_by | [end_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/end_outward_normal.md) |
| called_by | [offset_element](/crates/oxide-sketch/src/geom/offset_arc/offset_element.md) |
| called_by | [start_outward_normal](/crates/oxide-sketch/src/geom/offset_arc/start_outward_normal.md) |
