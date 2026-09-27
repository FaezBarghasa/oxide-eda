---
okf_version: "0.2"
type: Function
title: point_to_segment_distance
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/point_to_segment_distance
language: rust
---

# point_to_segment_distance

## Signature

```rust
fn point_to_segment_distance(p: Point, a: Point, b: Point) -> f64
```

## Source
Lines 667–669 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| called_by | [point_to_segment_dist](/crates/oxide-app/src/library/editor/footprint/canvas/geometry/point_to_segment_dist.md) |
| called_by | [hit_bus](/crates/oxide-app/src/schematic_runtime/hit_test/hit_bus.md) |
| called_by | [hit_wire](/crates/oxide-app/src/schematic_runtime/hit_test/hit_wire.md) |
