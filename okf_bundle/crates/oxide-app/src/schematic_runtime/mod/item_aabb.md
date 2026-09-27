---
okf_version: "0.2"
type: Function
title: item_aabb
resource: crates/oxide-app/src/schematic_runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/src/schematic_runtime/mod/item_aabb
language: rust
---

# item_aabb

## Signature

```rust
fn item_aabb(snapshot: &SchematicRenderSnapshot, item: &SelectedItem) -> Option<Aabb>
```

## Source
Lines 563–568 in `crates/oxide-app/src/schematic_runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [schematic_runtime](/crates/oxide-app/src/schematic_runtime/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| called_by | [draw_selection_overlay](/crates/oxide-app/src/schematic_runtime/selection/draw_selection_overlay.md) |
