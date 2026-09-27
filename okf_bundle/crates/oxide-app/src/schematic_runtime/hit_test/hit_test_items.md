---
okf_version: "0.2"
type: Function
title: hit_test_items
resource: crates/oxide-app/src/schematic_runtime/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/schematic_runtime/hit_test/hit_test_items
language: rust
---

# hit_test_items

## Signature

```rust
fn hit_test_items(snapshot: &SchematicRenderSnapshot, point: Point) -> Vec<SelectedItem>
```

## Source
Lines 62–77 in `crates/oxide-app/src/schematic_runtime/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test.md) |
| calls | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| calls | [hit_wire](/crates/oxide-app/src/schematic_runtime/hit_test/hit_wire.md) |
| calls | [hit_bus](/crates/oxide-app/src/schematic_runtime/hit_test/hit_bus.md) |
| called_by | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test.md) |
