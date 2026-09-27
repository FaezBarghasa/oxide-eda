---
okf_version: "0.2"
type: Function
title: offset_element
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc/offset_element
language: rust
---

# offset_element

## Signature

```rust
fn offset_element(el: PolyElement, d: f64, polygon_ccw: bool) -> PolyElement
```

## Source
Lines 173–202 in `crates/oxide-sketch/src/geom/offset_arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset_arc](/crates/oxide-sketch/src/geom/offset_arc.md) |
| calls | [unit_perp_outward](/crates/oxide-sketch/src/geom/offset_arc/unit_perp_outward.md) |
| called_by | [offset_arc_polyline](/crates/oxide-sketch/src/geom/offset_arc/offset_arc_polyline.md) |
