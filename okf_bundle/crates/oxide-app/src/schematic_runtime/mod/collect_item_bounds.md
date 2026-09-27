---
okf_version: "0.2"
type: Function
title: collect_item_bounds
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds
language: rust
---

# collect_item_bounds

## Signature

```rust
fn collect_item_bounds(snapshot: &SchematicRenderSnapshot) -> Vec<ItemBound>
```

## Source
Lines 402–561 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [symbol_body_aabb](/crates/oxide-app/src/schematic_runtime/mod/symbol_body_aabb.md) |
| calls | [text_prop_aabb](/crates/oxide-app/src/schematic_runtime/mod/text_prop_aabb.md) |
| calls | [label_aabb](/crates/oxide-app/src/schematic_runtime/mod/label_aabb.md) |
| calls | [note_aabb](/crates/oxide-app/src/schematic_runtime/mod/note_aabb.md) |
| calls | [drawing_aabb](/crates/oxide-app/src/schematic_runtime/mod/drawing_aabb.md) |
| called_by | [hit_test_items](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_items.md) |
| called_by | [hit_test_polygon](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_polygon.md) |
| called_by | [hit_test_rect_mode](/crates/oxide-app/src/schematic_runtime/hit_test/hit_test_rect_mode.md) |
| called_by | [item_aabb](/crates/oxide-app/src/schematic_runtime/mod/item_aabb.md) |
