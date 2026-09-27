---
okf_version: "0.2"
type: Function
title: slot_offsets
description: "Left-edge x of every slot (measured from the bar's own left edge)"
resource: crates/oxide-widgets/src/active_bar/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-widgets/src/active_bar/mod/slot_offsets
language: rust
---

# slot_offsets

Left-edge x of every slot (measured from the bar's own left edge)

## Signature

```rust
pub fn slot_offsets(items: &[ActiveBarItem<M>]) -> (Vec<f32>, f32)
```

## Type Parameters

- `M: 'static + Clone`

## Visibility

- `pub`

## Docstring

Left-edge x of every slot (measured from the bar's own left edge)
plus the bar's total width.

The bar is `Length::Shrink` and the caller centres it, so a consumer
that wants to anchor an overlay under button N has to know both
numbers: `bar_left = (window_w - total) / 2`, then
`offsets[n] + bar_left`. Deriving it here rather than in each
consumer is the point — the layout constants are private and the
item list is the only truth about what got drawn, so a bar that
gains, loses, or reorders a slot moves its dropdowns automatically.

## Source
Lines 105–116 in `crates/oxide-widgets/src/active_bar/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-widgets/src/active_bar/mod.md) |
| called_by | [menu_trigger_geometry](/crates/oxide-app/src/library/editor/footprint/unified_active_bar/menu_trigger_geometry.md) |
