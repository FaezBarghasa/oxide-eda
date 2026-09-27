---
okf_version: "0.2"
type: Function
title: sort_zone_stack
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/sort_zone_stack
language: rust
---

# sort_zone_stack

## Signature

```rust
pub(super) fn sort_zone_stack(zones: &mut [ZonePolygonInput])
```

## Visibility

- `pub(super)`

## Source
Lines 84–92 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [zone_layer_top_composite_key](/crates/oxide-renderer/src/pcb/emit/zone_layer_top_composite_key.md) |
| called_by | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
| called_by | [pcb_zone_sort_prefers_priority_then_connected_net_for_layer_top](/crates/oxide-renderer/src/pcb/mod/pcb_zone_sort_prefers_priority_then_connected_net_for_layer_top.md) |
