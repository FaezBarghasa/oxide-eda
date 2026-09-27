---
okf_version: "0.2"
type: Function
title: hit_test_rect_mode
resource: crates/oxide-app/src/schematic_runtime/hit_test.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/schematic_runtime/hit_test/hit_test_rect_mode
language: rust
---

# hit_test_rect_mode

## Signature

```rust
pub fn hit_test_rect_mode(
    snapshot: &SchematicRenderSnapshot,
    rect: &Aabb,
    mode: SelectionMode,
) -> Vec<SelectedItem>
```

## Visibility

- `pub`

## Source
Lines 37–60 in `crates/oxide-app/src/schematic_runtime/hit_test.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [hit_test](/crates/oxide-app/src/schematic_runtime/hit_test.md) |
| calls | [collect_item_bounds](/crates/oxide-app/src/schematic_runtime/mod/collect_item_bounds.md) |
| calls | [aabb_overlaps](/crates/oxide-app/src/schematic_runtime/mod/aabb_overlaps.md) |
| called_by | [handle_selection_request](/crates/oxide-app/src/app/handlers/selection_workflow/handle_selection_request.md) |
| called_by | [hit_test_rect_mode_distinguishes_inside_and_touching](/crates/oxide-app/src/schematic_runtime/tests/hit_test_rect_mode_distinguishes_inside_and_touching.md) |
