---
okf_version: "0.2"
type: Function
title: lock_sheet_pin_to_child_edge
resource: crates/oxide-engine/src/sheet.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/sheet/lock_sheet_pin_to_child_edge
language: rust
---

# lock_sheet_pin_to_child_edge

## Signature

```rust
pub(crate) fn lock_sheet_pin_to_child_edge(
    pin: &mut SheetPin,
    dx: f64,
    dy: f64,
    child_x: f64,
    child_y: f64,
    child_w: f64,
    child_h: f64,
)
```

## Visibility

- `pub(crate)`

## Source
Lines 76–141 in `crates/oxide-engine/src/sheet.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sheet](/crates/oxide-engine/src/sheet.md) |
| called_by | [move_selected_item](/crates/oxide-engine/src/transform/mod/move_selected_item.md) |
