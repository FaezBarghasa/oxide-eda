---
okf_version: "0.2"
type: Function
title: zone_layer_top_composite_key
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/zone_layer_top_composite_key
language: rust
---

# zone_layer_top_composite_key

## Signature

```rust
pub(super) fn zone_layer_top_composite_key(zone: &ZonePolygonInput) -> (u32, u8, u32)
```

## Visibility

- `pub(super)`

## Source
Lines 94–98 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [zone_connected_bucket](/crates/oxide-renderer/src/pcb/emit/zone_connected_bucket.md) |
| called_by | [sort_zone_stack](/crates/oxide-renderer/src/pcb/emit/sort_zone_stack.md) |
