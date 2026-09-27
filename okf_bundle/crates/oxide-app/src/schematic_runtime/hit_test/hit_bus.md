---
okf_version: "0.2"
type: Function
title: hit_bus
resource: crates/oxide-app/src/schematic_runtime/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/schematic_runtime/hit_test/hit_bus
language: rust
---

# hit_bus

## Signature

```rust
fn hit_bus(snapshot: &SchematicRenderSnapshot, uuid: uuid::Uuid, point: Point) -> bool
```

## Source
Lines 93–102 in `crates/oxide-app/src/schematic_runtime/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [point_to_segment_distance](/crates/oxide-app/src/schematic_runtime/mod/point_to_segment_distance.md) |
| called_by | [hit_test_items](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_items.md) |
