---
okf_version: "0.2"
type: Function
title: move_selected_item
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/move_selected_item
language: rust
---

# move_selected_item

## Signature

```rust
impl Engine { pub(super) fn move_selected_item(&mut self, item: &SelectedItem, dx: f64, dy: f64) -> bool }
```

## Visibility

- `pub(super)`

## Source
Lines 205–415 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [lock_sheet_pin_to_child_edge](/crates/oxide-engine/src/sheet/lock_sheet_pin_to_child_edge.md) |
| calls | [drawing_uuid](/crates/oxide-engine/src/transform/mod/drawing_uuid.md) |
