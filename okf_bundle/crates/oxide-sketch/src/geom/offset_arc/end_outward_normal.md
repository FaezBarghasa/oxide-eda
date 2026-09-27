---
okf_version: "0.2"
type: Function
title: end_outward_normal
resource: crates/oxide-sketch/src/geom/offset_arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-sketch/src/geom/offset_arc/end_outward_normal
language: rust
---

# end_outward_normal

## Signature

```rust
impl PolyElement { fn end_outward_normal(&self, polygon_ccw: bool) -> (f64, f64) }
```

## Source
Lines 85–99 in `crates/oxide-sketch/src/geom/offset_arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [offset_arc](/crates/oxide-sketch/src/geom/offset_arc.md) |
| calls | [unit_perp_outward](/crates/oxide-sketch/src/geom/offset_arc/unit_perp_outward.md) |
