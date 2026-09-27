---
okf_version: "0.2"
type: Function
title: zone_from_board_zone
resource: crates/oxide-renderer/src/pcb/emit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb/emit/zone_from_board_zone
language: rust
---

# zone_from_board_zone

## Signature

```rust
pub(super) fn zone_from_board_zone(
    zone: &Zone,
    source_order: usize,
    layer_ranks: &HashMap<String, u16>,
) -> Option<ZonePolygonInput>
```

## Visibility

- `pub(super)`

## Source
Lines 46–72 in `crates/oxide-renderer/src/pcb/emit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [emit](/crates/oxide-renderer/src/pcb/emit.md) |
| calls | [point_to_xy](/crates/oxide-renderer/src/pcb/emit/point_to_xy.md) |
| calls | [is_rule_area_zone](/crates/oxide-renderer/src/pcb/emit/is_rule_area_zone.md) |
| called_by | [from_board](/crates/oxide-renderer/src/pcb/mod/from_board.md) |
