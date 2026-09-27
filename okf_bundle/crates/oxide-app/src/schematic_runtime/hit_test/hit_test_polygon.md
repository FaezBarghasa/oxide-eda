---
okf_version: "0.2"
type: Function
title: hit_test_polygon
resource: crates/oxide-app/src/schematic_runtime/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/schematic_runtime/hit_test/hit_test_polygon
language: rust
---

# hit_test_polygon

## Signature

```rust
pub fn hit_test_polygon(
    snapshot: &SchematicRenderSnapshot,
    polygon: &[(f64, f64)],
) -> Vec<SelectedItem>
```

## Visibility

- `pub`

## Source
Lines 20–35 in `crates/oxide-app/src/schematic_runtime/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test.md) |
| calls | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| called_by | [dispatch_update](/crates/oxide-app/src/app/dispatch/mod/dispatch_update.md) |
| called_by | [handle_canvas_clicked](/crates/oxide-app/src/app/handlers/canvas/clicked/handle_canvas_clicked.md) |
| called_by | [hit_test_polygon_selects_wire_and_label_by_anchor](/crates/oxide-app/src/schematic_runtime/tests/hit_test_polygon_selects_wire_and_label_by_anchor.md) |
